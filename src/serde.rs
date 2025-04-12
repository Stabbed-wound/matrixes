use crate::Matrix;
use serde::{
    de::{self, SeqAccess, Visitor}, Deserialize, Deserializer, Serialize,
    Serializer,
};
use std::{fmt::Formatter, marker::PhantomData};

impl<T, const R: usize, const C: usize> Serialize for Matrix<T, R, C>
where
    T: Serialize,
{
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        serializer.serialize_newtype_struct(
            "Matrix",
            &self.0.iter().map(<[T; C]>::as_slice).collect::<Vec<_>>(),
        )
    }
}

struct MatrixVisitor<T, const R: usize, const C: usize> {
    marker: PhantomData<[[T; C]; R]>,
}

impl<'de, T, const R: usize, const C: usize> Visitor<'de> for MatrixVisitor<T, R, C>
where
    T: Deserialize<'de> + Copy,
{
    type Value = Matrix<T, R, C>;

    fn expecting(&self, formatter: &mut Formatter) -> std::fmt::Result {
        formatter.write_str("a sequence of sequences of equal length")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
    where
        A: SeqAccess<'de>,
    {
        let mut rows: Vec<[T; C]> = Vec::with_capacity(R);

        while let Some(row) = seq.next_element::<Box<[T]>>()? {
            rows.push(
                <[T; C]>::try_from(row.as_ref())
                    .map_err(|_| de::Error::invalid_length(row.len(), &self))?,
            );
        }

        rows.try_into()
            .map(|rows| Matrix(rows))
            .map_err(|error: Vec<[T; C]>| de::Error::invalid_length(error.len(), &self))
    }
}

impl<'de, T, const R: usize, const C: usize> Deserialize<'de> for Matrix<T, R, C>
where
    T: Deserialize<'de> + Copy,
{
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        deserializer.deserialize_seq(MatrixVisitor {
            marker: PhantomData::<[[T; C]; R]>,
        })
    }
}
