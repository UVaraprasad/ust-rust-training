pub fn add(a:i32,b:i32)->i32{
    a+b
}


#[cfg(test)]
mod tests{
use super::*;
    #[test]
    fn test_add_success(){
        let expected = 30;
        let actual = add(10,20);
        assert_eq!(actual,expected);
        assert_eq!(add(12,13),25);
    }

    #[test]
    fn test_add_fail(){
        let expected = 35;
        let actual = add(10,20);
        assert_ne!(actual,expected);
        assert_ne!(add(12,13),25);
    }
}

