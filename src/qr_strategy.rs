use crate::custom_error::CustomError;

pub type QrStrategyResult<T> = Result<T, CustomError>;

pub trait QRStrategy {
    fn generate(&self, output_file: &str) -> QrStrategyResult<()>;
}

pub struct QRContext {
    strategy: Box<dyn QRStrategy>,
}

impl QRContext {
    pub fn new(strategy: Box<dyn QRStrategy>) -> Self {
        QRContext { strategy }
    }

    pub fn generate_qr(&self, output_file: &str) -> QrStrategyResult<()> {
        self.strategy.generate(output_file)
    }
}
