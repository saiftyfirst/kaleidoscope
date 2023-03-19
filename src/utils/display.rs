/*
   First idea was to implement the following where structured_slice_print took an arbitrary container
   as its input instead of a slice -> This proved to be surprisingly difficult !!!

   Goal:
    - Loop through the Container using iter() without taking ownership

   Problem: (Vector as example of Container)
    - Found a way to do that using the following. However, this solution takes ownership of the Container.
      IntoIterator trait contains the into_ter() method which creates a owning Iterator. This is used
      by the iter method of Vector and hence calling iter on a Vector moves it!!

        fn _demonstration_X(x: impl IntoIterator)
        where Typ: std::fmt::Display
        {
            for i in x.iter().enumerate() {
                println!("{}", i.0);
             }
        }

    - Here is a funky way to achieve the Goal without slices
        - We define the function signature as follows:

            fn structured_slice_print<T>(args: &impl Container<Item=T>, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
                where T: std::fmt::Display

          We have used a trait Container which we need to define below. Container has the following requirements:
            - For a given lifetime 'a associated with a ref of Container, the ref has a valid IntoIterator to the refs of its items

                trait Container: for<'a> HasIterator<'a> {
                    ...
                }

            - Is a trait that provides an iter(&self) method with a return type of "some" IntoIter which owns a reference to underlying data
              and does not own it !!!

                fn iter(&self) -> <Self as HasIterator<'_>>::Iter;

           We need to define and implement the HasIterator.
             - HasIterator requires implementors to provide IntoIterator
             - HasIterator takes a lifetime parameter since we are dealing with references: 'a
             - HasIterator needs to attach lifetime to reference of self in order to keep bind the lifetimes of the refs
               pointed to by the iterators
             - Has a type of name Iter which returns an iterator to references of the underlying type in the Container

                trait HasIterator<'a, _Dummy=&'a Self>: IntoIterator
                // https://internals.rust-lang.org/t/question-regarding-one-of-your-posts/18536
                where
                    <Self as IntoIterator>::Item: 'a
                    // https://stackoverflow.com/questions/75777530/rust-template-parameter-with-lifetime-cannot-be-moved-inside-the-trait-closure/75777652#75777652
                {
                    type Iter: Iterator<Item=&'a <Self as IntoIterator>::Item>;
                }

                impl<'a, I> HasIter<'a> for I
                where
                    I: IntoIterator,
                    &'a I: IntoIterator<Item = &'a Self::Item>,
                {
                    type Iter = <&'a I as IntoIterator>::IntoIter;
                }

           Container Impl
                impl<T> Container for T
                where
                    T: IntoIterator,
                    for<'a> &'a T: IntoIterator<Item = &'a T::Item>,
                {
                    fn iter(&self) -> <Self as HasIter<'_>>::Iter {
                        self.into_iter()
                    }
                }
*/
pub fn structured_slice_print<T: std::fmt::Display>(args: &[T], f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
{
    for (i, arg) in args.iter().enumerate() {
        if i != 0 {
            write!(f, ", ")?;
        }
        write!(f, "{}", arg)?;
    }
    write!(f, ")")
}