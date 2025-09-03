/// Trait that ensures a reactive struct can be created from a non-reactive struct.
///
/// For example:
///
/// ```rust
/// struct User {
///     id: i32,
///     name: String
/// }
///
/// struct UserReactive {
///     id: RwSignal<i32>,
///     name: RwSignal<String>
/// }
///
/// impl ReactiveCapture for UserReactive {
///     // implement - required for ReactiveType.
/// }
///
/// impl AsReactive for User {
///     type ReactiveType = UserReactive;
///     fn as_reactive(self) -> UserReactive {
///         UserReactive {
///             id: RwSignal::new(self.id),
///             name: RwSignal::new(self.name)
///         }
///     }
/// }
/// ```
pub trait AsReactive {
    /// The type of the reactive struct. It must implement `ReactiveCapture`.
    type ReactiveType: ReactiveCapture;

    /// Wraps all fields of the struct into an `RwSignal`, returning a new
    /// "reactive" struct.
    ///
    /// Note that this method consumes the original struct and returns a new
    /// reactive struct.
    fn as_reactive(self) -> Self::ReactiveType;
}

/// Trait that ensures a reactive struct can be captured into a non-reactive struct.
/// The "captured" struct does not contain *any* reactive fields, meaning that it is not
/// wrapped in an `RwSignal`.
///
/// The captured struct should simply collect all the values from the reactive fields at
/// that point in time.
///
/// For example:
///
/// ```rust
/// struct User {
///     id: i32,
///     name: String
/// }
///
/// struct UserReactive {
///     id: RwSignal<i32>,
///     name: RwSignal<String>
/// }
///
/// impl AsReactive for User {
///     // implement - required for CaptureType.
/// }
///
/// impl ReactiveCapture for UserReactive {
///     type CaptureType = User;
///     fn capture(&self) -> User {
///         User {
///             id: self.id.get(),
///             name: self.name.get()
///         }
///     }
/// }
/// ```
pub trait ReactiveCapture {
    /// The type of the captured struct. It must implement `AsReactive`.
    type CaptureType: AsReactive;

    /// Captures the current state of the reactive struct into a non-reactive struct.
    fn capture(&self) -> Self::CaptureType;
}
