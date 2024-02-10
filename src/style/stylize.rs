use super::{modifier::Modifier, Color, Style, Styled};
use crate::text::Span;
use paste::paste;

/// Generates two methods for each color, one for setting the foreground color (`red()`, `blue()`,
/// etc) and one for setting the background color (`on_red()`, `on_blue()`, etc.). Each method sets
/// the color of the style to the corresponding color.
///
/// ```rust,ignore
/// color!(black);
///
/// // generates
///
/// #[doc = "Sets the foreground color to [`black`](Color::Black)."]
/// fn black(self) -> T {
///     self.fg(Color::Black)
/// }
///
/// #[doc = "Sets the background color to [`black`](Color::Black)."]
/// fn on_black(self) -> T {
///     self.bg(Color::Black)
/// }
/// ```
macro_rules! color {
    ( $color:ident ) => {
        paste! {
            #[doc = "Sets the foreground color to [`" $color "`](Color::" $color:camel ")."]
            #[must_use = concat!("`", stringify!($color), "` returns the modified style without modifying the original")]
            fn $color(self) -> T {
                self.fg(Color::[<$color:camel>])
            }

            #[doc = "Sets the background color to [`" $color "`](Color::" $color:camel ")."]
            #[must_use = concat!("`on_", stringify!($color), "` returns the modified style without modifying the original")]
            fn [<on_ $color>](self) -> T {
                self.bg(Color::[<$color:camel>])
            }
        }
    };
}

/// Generates a method for a modifier (`bold()`, `italic()`, etc.). Each method sets the modifier
/// of the style to the corresponding modifier.
///
/// # Examples
///
/// ```rust,ignore
/// modifier!(bold);
///
/// // generates
///
/// #[doc = "Adds the [`BOLD`](Modifier::BOLD) modifier."]
/// fn bold(self) -> T {
///     self.add_modifier(Modifier::BOLD)
/// }
///
/// #[doc = "Removes the [`BOLD`](Modifier::BOLD) modifier."]
/// fn not_bold(self) -> T {
///     self.remove_modifier(Modifier::BOLD)
/// }
/// ```
macro_rules! modifier {
    ( $modifier:ident ) => {
        paste! {
            #[doc = "Adds the [`" $modifier:upper "`](Modifier::" $modifier:upper ") modifier."]
            #[must_use = concat!("`", stringify!($modifier), "` returns the modified style without modifying the original")]
            fn [<$modifier>](self) -> T {
                self.add_modifier(Modifier::[<$modifier:upper>])
            }
        }

        paste! {
            #[doc = "Removes the [`" $modifier:upper "`](Modifier::" $modifier:upper ") modifier."]
            #[must_use = concat!("`not_", stringify!($modifier), "` returns the modified style without modifying the original")]
            fn [<not_ $modifier>](self) -> T {
                self.remove_modifier(Modifier::[<$modifier:upper>])
            }
        }
    };
}

/// An extension trait for styling objects.
///
/// For any type that implements `Stylize`, the provided methods in this trait can be used to style
/// the type further. This trait is automatically implemented for any type that implements the
/// [`Styled`] trait which e.g.: [`String`], [`&str`], [`Span`], [`Style`] and many Widget types.
///
/// This results in much more ergonomic styling of text and widgets. For example, instead of
/// writing:
///
/// ```rust,ignore
/// let text = Span::styled("Hello", Style::default().fg(Color::Red).bg(Color::Blue));
/// ```
///
/// You can write:
///
/// ```rust,ignore
/// let text = "Hello".red().on_blue();
/// ```
///
/// This trait implements a provided method for every color as both foreground and background
/// (prefixed by `on_`), and all modifiers as both an additive and subtractive modifier (prefixed
/// by `not_`). The `reset()` method is also provided to reset the style.
///
/// # Examples
/// ```
/// use zellij_widgets::prelude::*;
///
/// let span = "hello".red().on_blue().bold();
/// let line = Line::from(vec![
///     "hello".red().on_blue().bold(),
///     "world".green().on_yellow().not_bold(),
/// ]);
/// let paragraph = Paragraph::new(line).italic().underlined();
/// let block = Block::default().title("Title").borders(Borders::ALL).on_white().bold();
/// ```
pub trait Stylize<'a, T>: Sized {
    #[must_use = "`bg` returns the modified style without modifying the original"]
    fn bg(self, color: Color) -> T;
    #[must_use = "`fg` returns the modified style without modifying the original"]
    fn fg<S: Into<Color>>(self, color: S) -> T;
    #[must_use = "`reset` returns the modified style without modifying the original"]
    fn reset(self) -> T;
    #[must_use = "`add_modifier` returns the modified style without modifying the original"]
    fn add_modifier(self, modifier: Modifier) -> T;
    #[must_use = "`remove_modifier` returns the modified style without modifying the original"]
    fn remove_modifier(self, modifier: Modifier) -> T;

    color!(black);
    color!(red);
    color!(green);
    color!(yellow);
    color!(blue);
    color!(magenta);
    color!(cyan);
    color!(gray);
    color!(dark_gray);
    color!(white);

    modifier!(bold);
    modifier!(dim);
    modifier!(italic);
    modifier!(underlined);
    modifier!(slow_blink);
    modifier!(rapid_blink);
    modifier!(reversed);
    modifier!(hidden);
    modifier!(crossed_out);
}

impl<'a, T, U> Stylize<'a, T> for U
where
    U: Styled<Item = T>,
{
    fn bg(self, color: Color) -> T {
        let style = self.style().bg(color);
        self.set_style(style)
    }

    fn fg<S: Into<Color>>(self, color: S) -> T {
        let style = self.style().fg(color.into());
        self.set_style(style)
    }

    fn add_modifier(self, modifier: Modifier) -> T {
        let style = self.style().add_modifier(modifier);
        self.set_style(style)
    }

    fn remove_modifier(self, modifier: Modifier) -> T {
        let style = self.style().remove_modifier(modifier);
        self.set_style(style)
    }

    fn reset(self) -> T {
        self.set_style(Style::reset())
    }
}
