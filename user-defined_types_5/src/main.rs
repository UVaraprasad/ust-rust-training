fn main() {

    let mut x1= XYZBank::new(101, 24050.45);
    let mut y1 = XYZUPI::new(1231, 23478.45);
    let mut z1 = AmazonPay::new(1323, "myamazon101", 123123.34);
    make_payment(&mut x1, 1000.23);
    make_payment(&mut y1, 10324.0);
    make_payment(&mut z1, 100.3);
println!();
    make_payment1(&mut x1, 1000.23);
    make_payment1(&mut y1, 10324.0);
    make_payment1(&mut z1, 100.3);


}

fn make_payment(o: &mut impl Payment, amount: f32) {
    let bal = o.check_balance();
    println!("balance before payment:{}", bal);
    o.make_payment(amount);
    let bal = o.check_balance();
    println!("balance after payment:{}", bal);
}

fn make_payment1<T:Payment>(o: &mut T, amount: f32) {
    let bal = o.check_balance();
    println!("balance before payment:{}", bal);
    o.make_payment(amount);
    let bal = o.check_balance();
    println!("balance after payment:{}", bal);
}

// code is not designed well
fn make_paymentxyzbank(amount: f32, b: &mut XYZBank) {
    b.payment(amount);
    let bal = b.checkBalance();
    println!("balance:{}", bal);
}

fn make_paymentxyzupi(amount: f32, b: &mut XYZUPI) {
    b.upipayment(amount);
    let bal = b.checkupiBalance();
    println!("balance:{}", bal);
}

fn make_paymentamazon(amount: f32, b: &mut AmazonPay) {
    b.amazonpayment(amount);
    let bal = b.checkamzonBalance();
    println!("balance:{}", bal);
}
// end of code

trait Payment {
    fn check_balance(&self) -> f32;
    fn make_payment(&mut self, amount: f32) -> f32;
}

struct XYZBank {
    ac: i32,
    bal: f32,
}
struct XYZUPI {
    upiId: i32,
    bal: f32,
}

struct AmazonPay {
    upiId: i32,
    amazonid: String,
    bal: f32,
}


impl Payment for XYZBank {
    fn check_balance(&self) -> f32 {
        return self.bal;
    }

    fn make_payment(&mut self, amount: f32) -> f32 {
        let bal = self.checkBalance();
        if bal > amount {
            self.bal = self.bal - amount;
        }
        return self.bal;
    }
}

impl Payment for XYZUPI {
    fn check_balance(&self) -> f32 {
        return self.bal;
    }

    fn make_payment(&mut self, amount: f32) -> f32 {
        let bal = self.checkupiBalance();
        if bal > amount {
            self.bal = self.bal - amount;
        }
        return self.bal;
    }
}

impl Payment for AmazonPay {
    fn check_balance(&self) -> f32 {
        return self.bal;
    }

    fn make_payment(&mut self, amount: f32) -> f32 {
        let bal = self.checkamzonBalance();
        if bal > amount {
            self.bal = self.bal - amount;
        }
        return self.bal;
    }
}


impl XYZBank {
    fn checkBalance(&self) -> f32 {
        return self.bal;
    }

    fn payment(&mut self, amount: f32) -> f32 {
        let bal = self.checkBalance();
        if bal > amount {
            self.bal = self.bal - amount;
        }
        return self.bal;
    }

    fn new(ac: i32, bal: f32) -> Self {
        XYZBank { ac: ac, bal: bal }
    }
}

impl XYZUPI {

    fn new(id: i32, bal: f32) -> Self {
        XYZUPI { upiId: id, bal: bal }
    }
    fn checkupiBalance(&self) -> f32 {
        return self.bal;
    }

    fn upipayment(&mut self, amount: f32) -> f32 {
        let bal = self.checkupiBalance();
        if bal > amount {
            self.bal = self.bal - amount;
        }
        return self.bal;
    }
}

impl AmazonPay {

    fn new(id: i32,aid:&str, bal: f32) -> Self {
        AmazonPay { amazonid:aid.to_string(),upiId: id, bal: bal }
    }
    fn checkamzonBalance(&self) -> f32 {
        return self.bal;
    }

    fn amazonpayment(&mut self, amount: f32) -> f32 {
        let bal = self.checkamzonBalance();
        if bal > amount {
            self.bal = self.bal - amount;
        }
        return self.bal;
    }
}
