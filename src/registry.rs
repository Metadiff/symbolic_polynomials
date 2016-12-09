use monomial::Monomial;
use polynomial::Polynomial;

#[derive(Clone, Default)]
#[repr(C)]
pub struct Registry{
    pub id: u16
}

impl Registry {
    pub fn specific_monomial_variable(&mut self, id: u16) -> Monomial {
        Monomial{coefficient: 1, powers: vec![(id, 1)]}
    }

    pub fn new_monomial_variable(&mut self) -> Monomial {
        let id = self.id;
        self.id += 1;
        self.specific_monomial_variable(id)
    }

    pub fn specific_variable(&mut self, id: u16) -> Polynomial {
        Polynomial{monomials: vec![self.specific_monomial_variable(id)]}
    }

    pub fn new_variable(&mut self) -> Polynomial {
        Polynomial{monomials: vec![self.new_monomial_variable()]}
    }

    pub fn reset(&mut self) {
        self.id = 0;
    }
}