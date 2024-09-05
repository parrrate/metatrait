use super::base::BaseInstance;

pub type Results<E> = BaseInstance<crate::base::instances::result::Results<E>>;

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
        let x = Results::<()>::pure::<Is<_, Empty>>(0);
        let x = Results::<()>::map(x, |x| x + 1);
        let x = Results::<()>::map(x, |x| x + 1);
        let x = Results::<()>::map(x, |x| x + 1);
        let x = Results::<()>::map(x, |x| x + 1);
        let x = Results::<()>::map(x, |x| x + 1);
        let x = x.into_base();
        let x = x.unwrap();
        let x = x.into_that();
        assert_eq!(x, 5);
    }
}
