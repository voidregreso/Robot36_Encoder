fn main() {
    windows::build!{
        Windows::Win32::UI::WindowsAndMessaging::GetSystemMetrics,
        Windows::Win32::UI::WindowsAndMessaging::MessageBoxA
    };
}
