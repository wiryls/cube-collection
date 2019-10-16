using System.Windows;
using System.Windows.Controls;
using System.Windows.Controls.Primitives;
using System.Windows.Data;

namespace Editor.Views.AttachedProperties
{
    public class OverflowButton
    {
        public static readonly DependencyProperty AutoHiddenProperty;

        static OverflowButton()
        {
            AutoHiddenProperty = DependencyProperty.RegisterAttached(
                "AutoHidden",
                typeof(bool),
                typeof(OverflowButton),
                new PropertyMetadata(default(bool), OnAutoHiddenChanged)
            );
        }

        public static void SetAutoHidden(UIElement element, bool value)
        {
            element.SetValue(AutoHiddenProperty, value);
        }

        public static bool GetAutoHidden(UIElement element)
        {
            return (bool)element.GetValue(AutoHiddenProperty);
        }

        private static void OnAutoHiddenChanged(DependencyObject d, DependencyPropertyChangedEventArgs e)
        {
            void make(Control control)
            {
                if (control.Template.FindName("OverflowButton", control) is ButtonBase button)
                {
                    button.SetBinding(
                        UIElement.VisibilityProperty,
                        new Binding("IsEnabled")
                        {
                            RelativeSource = RelativeSource.Self,
                            Converter = new BooleanToVisibilityConverter()
                        }
                    );
                }
            }

            void handle(object sender, RoutedEventArgs args)
            {
                if (sender is Control control)
                {
                    control.Loaded -= handle;
                    make(control);
                }
            }

            if (d is Control it && e.NewValue is bool hidden)
            {
                if (hidden)
                {
                    if (it.IsLoaded)
                        make(it);
                    else
                        it.Loaded += handle;
                }
                else
                {
                    it.Loaded -= handle;
                    BindingOperations.ClearBinding(it, UIElement.VisibilityProperty);
                }
            }

            // Note:
            //
            // [WPF ToolBar: how to remove grip and overflow]
            // (https://stackoverflow.com/questions/1050953)
        }
    }
}
