use halo2_base::{self, QuantumCell, QuantumCell::Constant};
use halo2_base::{
    gates::{GateInstructions, RangeChip, RangeInstructions},
    utils::ScalarField,
    AssignedValue, Context,
};
use std::marker::PhantomData;

#[derive(Clone, Debug)]
pub struct IntegerDivisionChip<F: ScalarField> {
    pub range: RangeChip<F>,
    _marker: PhantomData<F>,
}

impl <'range, F: ScalarField> IntegerDivisionChip<F> {
    pub fn new(range: RangeChip<F>) -> Self {
        Self {
            range,
            _marker: PhantomData,
        }
    }

    // Implement integer division in Halo2!
    //
    // Public Inputs:
    // x: u32
    // y: u32
    // 
    // Public Output: quotient from integer division
    pub fn integer_division(
        &self,
        ctx: &mut Context<F>,
        x: u32,
        y: u32,
    ) -> AssignedValue<F> {
        let quo = x / y;
        let rem = x % y;

        let quo = ctx.load_witness(F::from(quo as u64));
        let rem = ctx.load_witness(F::from(rem as u64));
        let y = ctx.load_witness(F::from(y as u64));

        let reconstructed_x = self.range.gate.mul(ctx, quo, y);
        let reconstructed_x = self.range.gate.add(ctx, reconstructed_x, rem);
        let x = ctx.load_witness(F::from(x as u64));
        ctx.constrain_equal(&reconstructed_x, &x);

        self.range.range_check(ctx, quo, 32);
        self.range.range_check(ctx, rem, 32);



        quo
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use halo2_base::utils::testing::base_test;
    use halo2_base::halo2_proofs::halo2curves::bn256::Fr;

    #[test]
    fn test_integer_division() {
        env_logger::init();
        std::env::set_var("RUST_LOG", true.to_string());

        let k = 6;
        
        // Circuit inputs
        let x = 12;
        let y = 5;
        
        base_test().k(k as u32).lookup_bits(k - 1).run(|ctx, range| {
            let range = range.clone();
            let chip = IntegerDivisionChip::<Fr>::new(range);

            let result = chip.integer_division(
                ctx,
                x,
                y
            );
    
            let expected_result = 12 / 5;
    
            // Run test
            assert_eq!(result.value(), &Fr::from(expected_result));
        });
    }
}