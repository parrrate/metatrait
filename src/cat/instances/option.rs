use super::base::BaseInstance;

pub type Options = BaseInstance<crate::base::instances::option::Options>;

#[cfg(test)]
mod test {
    use crate::{
        cat::functor::*,
        traits::{
            base::BaseExt,
            empty::Empty,
            is::{Is, IsExt},
        },
    };

    use super::*;

    #[test]
    fn test() {
        let x = Options::pure::<Is<_, Empty>>(0);
        let x = Options::map(x, |x| x + 1);
        let x = Options::map(x, |x| x + 1);
        let x = Options::map(x, |x| x + 1);
        let x = Options::map(x, |x| x + 1);
        let x = Options::map(x, |x| x + 1);
        let x = x.into_base();
        let x = x.unwrap();
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
