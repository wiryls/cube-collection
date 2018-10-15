using FontAwesome.WPF;
using System.Windows;
using System.Windows.Controls;
using System.Windows.Controls.Primitives;

namespace Editor.Views.AttachedProperties
{
    public class Awesome
    {
        public static void SetIcon(UIElement element, FontAwesomeIcon? value)
        {
            if (value.HasValue == false)
                return;
            
            switch (element)
            {
                case ButtonBase it:
                    it.Content = new ImageAwesome
                    {
                        Icon = value.Value,
                        Width = it.FontSize,
                        Height = it.FontSize,
                        Foreground = it.Foreground
                    };
                    break;
                case MenuItem it:
                    it.Icon = new ImageAwesome
                    {
                        Icon = value.Value,
                        Width = it.FontSize,
                        Height = it.FontSize,
                        Foreground = it.Foreground
                    };
                    break;
            }
        }

        public static FontAwesomeIcon? GetIcon(UIElement element)
        {
            switch (element)
            {
                case ButtonBase it:
                    return it.Content is ImageAwesome
                        ? (it.Content as ImageAwesome).Icon
                        : default(FontAwesomeIcon?)
                        ;
                case MenuItem it:
                    return it.Icon is ImageAwesome
                        ? (it.Icon as ImageAwesome).Icon
                        : default(FontAwesomeIcon?)
                        ;
                default:
                    return default(FontAwesomeIcon?);
            }
        }

        // Note:
        //
        // [WPF Adding a custom property in a control]
        // (https://stackoverflow.com/a/18108958)
        // [C# 7.0: switch on System.Type]
        // (https://stackoverflow.com/a/43080709)
    }
}
