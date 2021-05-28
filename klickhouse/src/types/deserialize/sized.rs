use anyhow::*;
// use chrono::{Duration, NaiveDateTime, TimeZone};
use tokio::io::AsyncReadExt;
use uuid::Uuid;

use crate::{Date, DateTime, i256, io::ClickhouseRead, u256, values::Value};

use super::{Deserializer, DeserializerState, Type};


pub struct SizedDeserializer;

#[async_trait::async_trait]
impl Deserializer for SizedDeserializer {
    async fn read<R: ClickhouseRead>(type_: &Type, reader: &mut R, _state: &mut DeserializerState) -> Result<Value> {
        Ok(match type_ {
            Type::Int8 => Value::Int8(reader.read_i8().await?),
            Type::Int16 => Value::Int16(reader.read_i16_le().await?),
            Type::Int32 => Value::Int32(reader.read_i32_le().await?),
            Type::Int64 => Value::Int64(reader.read_i64_le().await?),
            Type::Int128 => Value::Int128(reader.read_i128_le().await?),
            Type::Int256 => {
                let mut buf = [0u8; 32];
                reader.read_exact(&mut buf[..]).await?;
                buf.reverse();
                Value::Int256(i256(buf))
            },
            Type::UInt8 => Value::UInt8(reader.read_u8().await?),
            Type::UInt16 => Value::UInt16(reader.read_u16_le().await?),
            Type::UInt32 => Value::UInt32(reader.read_u32_le().await?),
            Type::UInt64 => Value::UInt64(reader.read_u64_le().await?),
            Type::UInt128 => Value::UInt128(reader.read_u128_le().await?),
            Type::UInt256 => {
                let mut buf = [0u8; 32];
                reader.read_exact(&mut buf[..]).await?;
                buf.reverse();
                Value::UInt256(u256(buf))
            },
            Type::Float32 => Value::Float32(reader.read_u32_le().await?),
            Type::Float64 => Value::Float64(reader.read_u64_le().await?),
            Type::Decimal32(s) => Value::Decimal32(*s, reader.read_i32_le().await?),
            Type::Decimal64(s) => Value::Decimal64(*s, reader.read_i64_le().await?),
            Type::Decimal128(s) => Value::Decimal128(*s, reader.read_i128_le().await?),
            Type::Decimal256(s) => {
                let mut buf = [0u8; 32];
                reader.read_exact(&mut buf[..]).await?;
                buf.reverse();
                Value::Decimal256(*s, i256(buf))
            }
            Type::Uuid => Value::Uuid({
                let n1 = reader.read_u64_le().await?;
                let n2 = reader.read_u64_le().await?;
                Uuid::from_u128((n1 as u128) << 64 | n2 as u128)
            }),
            Type::Date => Value::Date(Date(reader.read_u16_le().await?)),
            Type::DateTime(tz) => Value::DateTime(DateTime(*tz, reader.read_u32_le().await?)),
            Type::DateTime64(precision, tz) => {
                let raw = reader.read_u64_le().await?;
                Value::DateTime64(*tz, *precision, raw)
            },
            Type::Enum8(_) => Value::Enum8(reader.read_u8().await?),
            Type::Enum16(_) => Value::Enum16(reader.read_u16_le().await?),
            _ => unimplemented!(),
        })
    }
}