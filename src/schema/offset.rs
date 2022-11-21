/// The offset of a field or data member.
#[derive(Copy)]
pub struct Offset<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    unit: &'dwarf crate::gimli::Unit<R, usize>,
    inner: OffsetInner<R>,
}

#[derive(Copy)]
enum OffsetInner<R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    Udata(u64),
    Expression(crate::gimli::read::Expression<R>),
}

impl<'dwarf, R> Offset<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    pub(crate) fn zero(unit: &'dwarf crate::gimli::Unit<R, usize>) -> Self {
        Self {
            unit,
            inner: OffsetInner::Udata(0),
        }
    }

    /// Construct a new `Offset` from a given `entry`'s `DW_AT_data_member_location` attribute.
    pub(crate) fn from_die<'entry>(
        unit: &'dwarf crate::gimli::Unit<R, usize>,
        entry: &'entry crate::gimli::DebuggingInformationEntry<'dwarf, 'dwarf, R>,
    ) -> Result<Option<Self>, crate::Error> {
        let maybe_location = entry.attr_value(crate::gimli::DW_AT_data_member_location)?;
        Ok(if let Some(location) = maybe_location {
            let inner = if let Some(offset) = location.udata_value() {
                OffsetInner::Udata(offset)
            } else if let Some(expression) = location.exprloc_value() {
                OffsetInner::Expression(expression)
            } else {
                return Err(crate::ErrorKind::ValueMismatch.into());
            };
            Some(Self { unit, inner })
        } else {
            None
        })
    }

    pub fn address(self, start: u64) -> Result<u64, crate::Error> {
        match self.inner {
            OffsetInner::Udata(offset) => Ok(start + offset),
            OffsetInner::Expression(expression) => {
                let mut eval = expression.evaluation(self.unit.encoding());
                eval.set_initial_value(start);
                if let crate::gimli::EvaluationResult::Complete = eval.evaluate()? {
                    let result = eval.result();
                    match result[..] {
                        [crate::gimli::Piece {
                            size_in_bits: None,
                            bit_offset: None,
                            location: crate::gimli::Location::Address { address },
                        }] => Ok(address),
                        _ => unimplemented!("unsupported evaluation result {:?}", result,),
                    }
                } else {
                    unimplemented!("unsupported evaluation result");
                }
            }
        }
    }
}

impl<'dwarf, R> Clone for Offset<'dwarf, R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn clone(&self) -> Self {
        Self {
            unit: self.unit,
            inner: self.inner.clone(),
        }
    }
}

impl<R> Clone for OffsetInner<R>
where
    R: crate::gimli::Reader<Offset = usize>,
{
    fn clone(&self) -> Self {
        match self {
            Self::Udata(offset) => Self::Udata(*offset),
            Self::Expression(expr) => Self::Expression(expr.clone()),
        }
    }
}
