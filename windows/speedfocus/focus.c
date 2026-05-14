/* Vibe-coded with Gemini.
 * https://gemini.google.com/share/d9a2f7053a3b
 * https://gemini.google.com/share/20504a7f280b
 */

/* Resources:
 * https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes
 * https://godbolt.org/z/61W4vhzs3
 * https://stackoverflow.com/a/72620669
 * https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-bringwindowtotop
 * https://winlibs.com/
 * https://learn.microsoft.com/en-us/cpp/windows/walkthrough-creating-windows-desktop-applications-cpp?view=msvc-170
 *
 * https://learn.microsoft.com/en-us/windows/win32/winmsg/using-messages-and-message-queues
 * https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-sendmessage
 * https://rovecoder.net/article/win32/handling-events
 *
 *
 * Rust resources:
 * https://crates.io/crates/windows
 * https://github.com/microsoft/windows-rs
 * https://github.com/sigoden/window-switcher/blob/main/src/main.rs
 * https://github.com/ryancerium/grist/tree/master/src
 * https://github.com/rodrigocfd/winsafe
 *
 * */

/* Set path:
 * SET PATH=C:\Users\jon\Downloads\winlibs-x86_64-posix-seh-gcc-15.2.0-mingw-w64msvcrt-13.0.0-r1\mingw64\bin;%PATH%
 */
/* Building:
C:
cd C:\users\jon\source\repos\focus
SET PATH=C:\Tools\mingw64\bin;%PATH%
gcc -o focus.exe focus.c -luser32 -lole32 -loleaut32 -luuid -mwindows
copy /y focus.exe ..\..\..

 */

// This program uses a low-level keyboard hook to capture keystrokes.
// Compile with a C compiler like GCC (MinGW-w64).
// Example compilation command:
// gcc -o focus.exe focus.c -luser32 -lole32 -loleaut32 -luuid -mwindows

#include <stdio.h>
#include <windows.h>
#include <tchar.h>
#include <string.h>

#include <objbase.h>
#include <uiautomation.h>

#define WM_TRAYICON_MESSAGE (WM_USER + 1)
#define WM_QUIT_PROGRAM (WM_USER + 2)
#define WM_HOTKEY_TRIGGER (WM_USER + 3)

IUIAutomation* pAutomation = NULL;
HHOOK g_keyboard_hook = NULL;
HWND g_hwnd = NULL;

// Struct to hold a single hotkey configuration
typedef struct {
    int key_code;
    const TCHAR* app_name;
} Shortcut;

/* Programs I use:
 * - emacs
 * - teams
 * - edge/chrome
 * - firefox
 * - vscode
 * - explorer?
 */

// Static array of structs for all configured shortcuts
static const Shortcut shortcuts[] = {
    {0x45, _T("emax")},               /* E */
    {0x43, _T("Microsoft? Edge")},    /* C had to use a debugger for that one */
    {0x54, _T("Microsoft Teams")},    /* T */
    {0x46, _T("firefox")},            /* F */
    {0x56, _T("visual studio code")}, /* V */
    {0x4B, _T("jon@wslhostname")},   /* K */
    {0x48, _T("Minimize")},           /* H */
    {0x51, _T("Quit")}                /* Q */
};

BOOL MoveWindowToForeground(IUIAutomation* pAutomation, HWND hWnd) {
    IUIAutomationElement* element = NULL;
    HRESULT result = pAutomation->lpVtbl->ElementFromHandle(pAutomation, hWnd, &element);
    if (FAILED(result))
        return FALSE;

    result = element->lpVtbl->SetFocus(element);

    element->lpVtbl->Release(element);
    return SUCCEEDED(result);
}

// Global variables to store the found window handle and the search keyword.
// This is used by the EnumWindows callback function.
HWND found_hwnd = NULL;
TCHAR keyword_to_find[256];

BOOL CALLBACK EnumWindowsProc(HWND hwnd, LPARAM lParam) {
    TCHAR window_title[256];

    // Check if the window is visible
    if (IsWindowVisible(hwnd)) {
        // Get the window title
        if (GetWindowText(hwnd, window_title, 256) > 0) {
            // Convert both strings to lowercase for a case-insensitive comparison
            CharLower(window_title);
            CharLower(keyword_to_find);

            // Check if the window title contains the keyword
            if (_tcsstr(window_title, keyword_to_find) != NULL) {
                found_hwnd = hwnd;
                return FALSE; // Stop enumeration
            }
        }
    }
    return TRUE; // Continue enumeration
}

void focus_window_by_title(const TCHAR* keyword) {
    _tcscpy(keyword_to_find, keyword);
    found_hwnd = NULL;

    // Enumerate all top-level windows using the callback function
    EnumWindows(EnumWindowsProc, 0);

    if (found_hwnd) {
        /* SendMessage(found_hwnd, WM_SYSCOMMAND, SC_MINIMIZE, 0); */
        /* Sleep(10); */
        SendMessage(found_hwnd, WM_SYSCOMMAND, SC_RESTORE, 0);
        /* Sleep(10); */
        /* BringWindowToTop(found_hwnd); */
        MoveWindowToForeground(pAutomation, found_hwnd);
    }
}

// The low-level keyboard hook procedure
LRESULT CALLBACK LowLevelKeyboardProc(int nCode, WPARAM wParam, LPARAM lParam) {
    if (nCode == HC_ACTION && (wParam == WM_KEYDOWN || wParam == WM_SYSKEYDOWN)) {
        PKBDLLHOOKSTRUCT p = (PKBDLLHOOKSTRUCT)lParam;

        BOOL is_win_key_down = (GetAsyncKeyState(VK_LWIN) & 0x8000) || (GetAsyncKeyState(VK_RWIN) & 0x8000);

        if (is_win_key_down) {
            for (int i = 0; i < sizeof(shortcuts) / sizeof(shortcuts[0]); ++i) {
                if (p->vkCode == shortcuts[i].key_code) {
                    PostMessage(g_hwnd, WM_HOTKEY_TRIGGER, shortcuts[i].key_code, 0);
                    return 1; // Return non-zero to prevent system from processing the event
                }
            }
        }
    }

    return CallNextHookEx(g_keyboard_hook, nCode, wParam, lParam);
}

LRESULT CALLBACK WndProc(HWND hWnd, UINT message, WPARAM wParam, LPARAM lParam) {
    switch (message) {
        case WM_CREATE: {
            NOTIFYICONDATA nid = { sizeof(nid) };
            nid.hWnd = hWnd;
            nid.uID = 1;
            nid.uFlags = NIF_ICON | NIF_TIP | NIF_MESSAGE;
            nid.uCallbackMessage = WM_TRAYICON_MESSAGE;
            nid.hIcon = LoadIcon(NULL, IDI_APPLICATION);
            _tcscpy(nid.szTip, _T("Speed focus"));
            Shell_NotifyIcon(NIM_ADD, &nid);
            break;
        }

        case WM_TRAYICON_MESSAGE: {
            switch (lParam) {
                case WM_LBUTTONUP:
                case WM_RBUTTONUP:
                    PostQuitMessage(0);
                    break;
            }
            break;
        }

        case WM_COMMAND: {
            if (LOWORD(wParam) == WM_QUIT_PROGRAM) {
                // Unhook the keyboard hook to prevent any new hotkey messages from being generated.
                if (g_keyboard_hook) {
                    UnhookWindowsHookEx(g_keyboard_hook);
                    g_keyboard_hook = NULL;
                }

                MSG msg_to_check;
                // Peek and remove all pending WM_HOTKEY_TRIGGER messages
                while (PeekMessage(&msg_to_check, hWnd, WM_HOTKEY_TRIGGER, WM_HOTKEY_TRIGGER, PM_REMOVE)) {
                    // Do nothing, just drain the queue
                }
                PostQuitMessage(0);
            }
            break;
        }

        case WM_HOTKEY_TRIGGER: {
            for (int i = 0; i < sizeof(shortcuts) / sizeof(shortcuts[0]); ++i) {
                if (wParam == shortcuts[i].key_code) {
                    if (_tcscmp(shortcuts[i].app_name, _T("Quit")) == 0) {
                        PostQuitMessage(0);
                    } else if (_tcscmp(shortcuts[i].app_name, _T("Minimize")) == 0) {
                        HWND hForegroundWnd = GetForegroundWindow();
                        // Check if the foreground window is our own utility window, we don't want to minimize it.
                        if (hForegroundWnd && hForegroundWnd != g_hwnd) {
                            // Use WM_SYSCOMMAND to minimize the window, delegating focus change to the shell.
                            SendMessage(hForegroundWnd, WM_SYSCOMMAND, SC_MINIMIZE, 0);
                        }
                    } else {
                        focus_window_by_title(shortcuts[i].app_name);
                    }
                }
            }
            break;
        }

        case WM_DESTROY: {
            NOTIFYICONDATA nid = { sizeof(nid) };
            nid.hWnd = hWnd;
            nid.uID = 1;
            Shell_NotifyIcon(NIM_DELETE, &nid);

            if (g_keyboard_hook) {
                UnhookWindowsHookEx(g_keyboard_hook);
            }
            pAutomation->lpVtbl->Release(pAutomation);
            CoUninitialize();
            break;
        }
    }
    return DefWindowProc(hWnd, message, wParam, lParam);
}

int WINAPI WinMain(HINSTANCE hInstance, HINSTANCE hPrevInstance, LPSTR lpCmdLine, int nCmdShow) {
    CoInitialize(NULL);

    HRESULT result = CoCreateInstance(&CLSID_CUIAutomation, NULL, CLSCTX_INPROC_SERVER, &IID_IUIAutomation, (LPVOID*)&pAutomation);
    if (FAILED(result))
        return 1;

    WNDCLASS wc = { 0 };
    wc.lpfnWndProc = WndProc;
    wc.hInstance = hInstance;
    wc.lpszClassName = _T("SpeedFocusClass");
    RegisterClass(&wc);

    g_hwnd = CreateWindowEx(0, _T("SpeedFocusClass"), _T(""), 0, 0, 0, 0, 0, HWND_MESSAGE, NULL, hInstance, NULL);

    g_keyboard_hook = SetWindowsHookEx(
        WH_KEYBOARD_LL,
        LowLevelKeyboardProc,
        GetModuleHandle(NULL),
        0
    );

    if (g_keyboard_hook == NULL) {
        return 1;
    }

    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0)) {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
    }

    return 0;
}
