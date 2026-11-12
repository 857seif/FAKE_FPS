#![windows_subsystem = "windows"]

use std::mem::{size_of, zeroed};
use std::path::{Path, PathBuf};
use std::ptr;
use std::sync::atomic::{AtomicBool, AtomicU64, Ordering};
use std::sync::Mutex;

type Hinst = *mut core::ffi::c_void;
type Hwnd = *mut core::ffi::c_void;
type Handle = *mut core::ffi::c_void;
type Hdc = *mut core::ffi::c_void;
type Hfont = *mut core::ffi::c_void;
type Hbrush = *mut core::ffi::c_void;
type Hgdiobj = *mut core::ffi::c_void;

#[repr(C)]
struct WndClassW {
    style: u32,
    lpfn_wnd_proc: Option<unsafe extern "system" fn(Hwnd, u32, usize, isize) -> isize>,
    cb_cls_extra: i32,
    cb_wnd_extra: i32,
    h_instance: Hinst,
    h_icon: Handle,
    h_cursor: Handle,
    hbr_background: Handle,
    menu_name: *const u16,
    class_name: *const u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Point {
    x: i32,
    y: i32,
}

#[repr(C)]
struct Msg {
    hwnd: Hwnd,
    message: u32,
    w_param: usize,
    l_param: isize,
    time: u32,
    pt: Point,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Rect {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[repr(C)]
struct PaintStruct {
    hdc: Hdc,
    erase: i32,
    paint: Rect,
    restore: i32,
    inc_update: i32,
    reserved: [u8; 32],
}

#[repr(C)]
struct StartupInfoW {
    cb: u32,
    reserved: *mut u16,
    desktop: *mut u16,
    title: *mut u16,
    x: u32,
    y: u32,
    x_size: u32,
    y_size: u32,
    x_chars: u32,
    y_chars: u32,
    fill: u32,
    flags: u32,
    show: u16,
    reserved2: u16,
    reserved2_ptr: *mut u8,
    std_input: Handle,
    std_output: Handle,
    std_error: Handle,
}

#[repr(C)]
struct ProcessInformation {
    h_process: Handle,
    h_thread: Handle,
    process_id: u32,
    thread_id: u32,
}

#[repr(C)]
struct OpenFileNameW {
    struct_size: u32,
    owner: Hwnd,
    instance: Hinst,
    filter: *const u16,
    custom_filter: *const u16,
    max_custom_filter: u32,
    filter_index: u32,
    file: *mut u16,
    max_file: u32,
    file_title: *mut u16,
    max_file_title: u32,
    initial_dir: *mut u16,
    title: *const u16,
    flags: u32,
    filter_index_dup: u16,
    custom_filter_dup: u16,
    template_name: *const u16,
    custom_data: *mut core::ffi::c_void,
    hook: *mut core::ffi::c_void,
    template_name_dup: *const u16,
    reserved: usize,
    dw_reserved: u32,
    flags_ex: u32,
}

#[repr(C)]
struct ProcessEntry32W {
    dw_size: u32,
    cnt_usage: u32,
    th32_process_id: u32,
    th32_default_heap_id: usize,
    th32_module_id: u32,
    cnt_threads: u32,
    th32_parent_process_id: u32,
    pc_pri_class_base: i32,
    dw_flags: u32,
    sz_exe_file: [u16; 260],
}

#[repr(C)]
struct MemInfo {
    base: usize,
    alloc_base: usize,
    alloc_protect: u32,
    partition_id: u16,
    pad: u16,
    region_size: usize,
    state: u32,
    protect: u32,
    mem_type: u32,
    pad2: u32,
}

#[repr(C)]
struct TriVertex {
    x: i32,
    y: i32,
    red: u16,
    green: u16,
    blue: u16,
    alpha: u16,
}

#[repr(C)]
struct GradientRect {
    upper_left: u32,
    lower_right: u32,
}

#[repr(C)]
struct IconInfo {
    f_icon: i32,
    x_hot: u32,
    y_hot: u32,
    h_mask: Handle,
    h_color: Handle,
}

#[repr(C)]
struct DrawItemStruct {
    ctl_type: u32,
    ctl_id: u32,
    item_id: u32,
    item_action: u32,
    item_state: u32,
    hwnd_item: Hwnd,
    hdc: Hdc,
    rc_item: Rect,
    item_data: usize,
}

#[link(name = "user32")]
unsafe extern "system" {
    fn RegisterClassW(c: *const WndClassW) -> u16;
    fn LoadCursorW(instance: Hinst, name: *const u16) -> Handle;
    fn GetSysColorBrush(index: i32) -> Handle;
    fn CreateWindowExW(
        ex_style: u32,
        class: *const u16,
        name: *const u16,
        style: u32,
        x: i32,
        y: i32,
        w: i32,
        h: i32,
        parent: Hwnd,
        menu: Handle,
        instance: Hinst,
        param: *mut core::ffi::c_void,
    ) -> Hwnd;
    fn DefWindowProcW(hwnd: Hwnd, msg: u32, w: usize, l: isize) -> isize;
    fn ShowWindow(hwnd: Hwnd, cmd: i32) -> i32;
    fn UpdateWindow(hwnd: Hwnd) -> i32;
    fn GetMessageW(msg: *mut Msg, hwnd: Hwnd, min: u32, max: u32) -> i32;
    fn TranslateMessage(msg: *const Msg) -> i32;
    fn DispatchMessageW(msg: *const Msg) -> isize;
    fn PostQuitMessage(code: i32);
    fn SetWindowTextW(hwnd: Hwnd, text: *const u16) -> i32;
    fn GetWindowTextW(hwnd: Hwnd, text: *mut u16, max: i32) -> i32;
    fn SendMessageW(hwnd: Hwnd, msg: u32, w: usize, l: isize) -> isize;
    fn MessageBoxW(hwnd: Hwnd, text: *const u16, caption: *const u16, kind: u32) -> i32;
    fn SetForegroundWindow(hwnd: Hwnd) -> i32;
    fn AdjustWindowRect(rect: *mut Rect, style: u32, menu: i32) -> i32;
    fn GetClientRect(hwnd: Hwnd, rect: *mut Rect) -> i32;
    fn GetSystemMetrics(index: i32) -> i32;
    fn GetDC(hwnd: Hwnd) -> Hdc;
    fn ReleaseDC(hwnd: Hwnd, hdc: Hdc) -> i32;
    fn CreateIconIndirect(info: *const IconInfo) -> Handle;
    fn InvalidateRect(hwnd: Hwnd, rect: *const Rect, erase: i32) -> i32;
    fn BeginPaint(hwnd: Hwnd, ps: *mut PaintStruct) -> Hdc;
    fn EndPaint(hwnd: Hwnd, ps: *const PaintStruct) -> i32;
    fn FillRect(hdc: Hdc, rect: *const Rect, brush: Handle) -> i32;
    fn DrawTextW(hdc: Hdc, text: *const u16, len: i32, rect: *mut Rect, flags: u32) -> i32;
    fn SetCapture(hwnd: Hwnd) -> Hwnd;
    fn ReleaseCapture() -> i32;
    fn SetTimer(hwnd: Hwnd, id: usize, ms: u32, func: *mut core::ffi::c_void) -> usize;
    fn DrawIconEx(
        hdc: Hdc,
        x: i32,
        y: i32,
        icon: Handle,
        cx: i32,
        cy: i32,
        istep_if_animated: u32,
        flicker_free_brush: Handle,
        istep: u32,
        flags: u32,
    ) -> i32;
    fn GetCursorPos(p: *mut Point) -> i32;
}

#[link(name = "shell32")]
unsafe extern "system" {
    fn ExtractIconExW(
        file: *const u16,
        index: i32,
        large: *mut Handle,
        small: *mut Handle,
        icons: u32,
    ) -> u32;
    fn DestroyIcon(icon: Handle) -> i32;
}

#[link(name = "comdlg32")]
unsafe extern "system" {
    fn GetOpenFileNameW(ofn: *mut OpenFileNameW) -> i32;
    fn CommDlgExtendedError() -> u32;
}

#[link(name = "msimg32")]
unsafe extern "system" {
    fn GradientFill(
        hdc: Hdc,
        vertices: *const TriVertex,
        n_vertices: u32,
        rects: *const GradientRect,
        n_rects: u32,
        mode: u32,
    ) -> i32;
}

#[link(name = "dwmapi")]
unsafe extern "system" {
    fn DwmSetWindowAttribute(hwnd: Hwnd, attr: u32, value: *const core::ffi::c_void, size: u32) -> i32;
}

#[link(name = "advapi32")]
unsafe extern "system" {
    fn RegCreateKeyExW(
        root: usize,
        sub: *const u16,
        reserved: u32,
        class: *const u16,
        options: u32,
        access: u32,
        sec: *mut core::ffi::c_void,
        out: *mut usize,
        disp: *mut u32,
    ) -> i32;
    fn RegQueryValueExW(key: usize, name: *const u16, reserved: *mut u32, kind: *mut u32, data: *mut u8, len: *mut u32) -> i32;
    fn RegSetValueExW(key: usize, name: *const u16, reserved: u32, kind: u32, data: *const u8, len: u32) -> i32;
    fn RegCloseKey(key: usize) -> i32;
}

#[link(name = "gdi32")]
unsafe extern "system" {
    fn GetStockObject(id: i32) -> Handle;
    fn SetBkMode(hdc: Hdc, mode: i32) -> i32;
    fn CreateSolidBrush(color: u32) -> Hbrush;
    fn CreateFontW(
        height: i32,
        width: i32,
        escapement: i32,
        orientation: i32,
        weight: i32,
        italic: u32,
        underline: u32,
        strike: u32,
        charset: u32,
        out_precision: u32,
        clip_precision: u32,
        quality: u32,
        pitch: u32,
        face: *const u16,
    ) -> Hfont;
    fn SelectObject(hdc: Hdc, obj: Hgdiobj) -> Hgdiobj;
    fn DeleteObject(obj: Hgdiobj) -> i32;
    fn SetTextColor(hdc: Hdc, color: u32) -> u32;
    fn SetBkColor(hdc: Hdc, color: u32) -> u32;
    fn RoundRect(hdc: Hdc, l: i32, t: i32, r: i32, b: i32, w: i32, h: i32) -> i32;
    fn Ellipse(hdc: Hdc, l: i32, t: i32, r: i32, b: i32) -> i32;
    fn CreateCompatibleDC(hdc: Hdc) -> Hdc;
    fn CreateCompatibleBitmap(hdc: Hdc, w: i32, h: i32) -> Handle;
    fn CreateBitmap(w: i32, h: i32, planes: u32, bits: u32, data: *const core::ffi::c_void) -> Handle;
    fn BitBlt(dst: Hdc, x: i32, y: i32, w: i32, h: i32, src: Hdc, sx: i32, sy: i32, rop: u32) -> i32;
}

#[link(name = "kernel32")]
unsafe extern "system" {
    fn GetModuleHandleW(name: *const u16) -> Hinst;
    fn CreateProcessW(
        app: *const u16,
        cmd: *const u16,
        proc_attr: *mut core::ffi::c_void,
        thread_attr: *mut core::ffi::c_void,
        inherit: i32,
        flags: u32,
        env: *mut core::ffi::c_void,
        cwd: *const u16,
        si: *mut StartupInfoW,
        pi: *mut ProcessInformation,
    ) -> i32;
    fn WaitForSingleObject(obj: Handle, ms: u32) -> u32;
    fn CloseHandle(obj: Handle) -> i32;
    fn OpenProcess(access: u32, inherit: i32, pid: u32) -> Handle;
    fn CreateToolhelp32Snapshot(flags: u32, pid: u32) -> Handle;
    fn Process32FirstW(snap: Handle, entry: *mut ProcessEntry32W) -> i32;
    fn Process32NextW(snap: Handle, entry: *mut ProcessEntry32W) -> i32;
    fn QueryFullProcessImageNameW(proc: Handle, flags: u32, buf: *mut u16, size: *mut u32) -> i32;
    fn VirtualQueryEx(proc: Handle, addr: *const core::ffi::c_void, info: *mut MemInfo, len: usize) -> usize;
    fn ReadProcessMemory(proc: Handle, addr: *const core::ffi::c_void, buf: *mut core::ffi::c_void, size: usize, read: *mut usize) -> i32;
    fn WriteProcessMemory(proc: Handle, addr: *mut core::ffi::c_void, buf: *const core::ffi::c_void, size: usize, written: *mut usize) -> i32;
    fn VirtualProtectEx(proc: Handle, addr: *mut core::ffi::c_void, size: usize, protect: u32, old: *mut u32) -> i32;
    fn FlushInstructionCache(proc: Handle, addr: *const core::ffi::c_void, size: usize) -> i32;
    fn OpenFileMappingW(access: u32, inherit: i32, name: *const u16) -> Handle;
    fn MapViewOfFile(map: Handle, access: u32, off_hi: u32, off_lo: u32, bytes: usize) -> *mut core::ffi::c_void;
    fn UnmapViewOfFile(base: *const core::ffi::c_void) -> i32;
    fn Sleep(ms: u32);
}

const WM_CREATE: u32 = 0x0001;
const WM_DESTROY: u32 = 0x0002;
const WM_CLOSE: u32 = 0x0010;
const WM_PAINT: u32 = 0x000F;
const WM_ERASEBKGND: u32 = 0x0014;
const WM_DRAWITEM: u32 = 0x002B;
const WM_COMMAND: u32 = 0x0111;
const WM_TIMER: u32 = 0x0113;
const WM_MOUSEMOVE: u32 = 0x0200;
const WM_LBUTTONDOWN: u32 = 0x0201;
const WM_LBUTTONUP: u32 = 0x0202;
const WM_LBUTTONDBLCLK: u32 = 0x0203;
const WM_RBUTTONDOWN: u32 = 0x0204;
const WM_CTLCOLOREDIT: u32 = 0x0133;
const WM_SETICON: u32 = 0x0080;
const WM_SETFONT: u32 = 0x0030;

const WS_CHILD: u32 = 0x4000_0000;
const WS_VISIBLE: u32 = 0x1000_0000;
const WS_BORDER: u32 = 0x0080_0000;
const WS_CAPTION: u32 = 0x00C0_0000;
const WS_SYSMENU: u32 = 0x0008_0000;
const WS_MINIMIZEBOX: u32 = 0x0002_0000;
const ES_AUTOHSCROLL: u32 = 0x0080;
const BS_OWNERDRAW: u32 = 0x0000_000B;
const ODS_SELECTED: u32 = 0x0001;
const CS_DBLCLKS: u32 = 0x0008;

const SW_SHOW: i32 = 5;

const NULL_PEN: i32 = 8;
const MB_ICONERROR: u32 = 0x0000_0010;

const DT_LEFT: u32 = 0x0000;
const DT_CENTER: u32 = 0x0001;
const DT_RIGHT: u32 = 0x0002;
const DT_VCENTER: u32 = 0x0004;
const DT_SINGLELINE: u32 = 0x0020;
const DT_END_ELLIPSIS: u32 = 0x8000;

const ES_CENTER: u32 = 0x0001;
const TH32CS_SNAPPROCESS: u32 = 0x0000_0002;
const PROCESS_QUERY_LIMITED: u32 = 0x0000_1000;
const PROCESS_PATCH: u32 = 0x0010_0438;
const PAGE_EXECUTE_READWRITE: u32 = 0x40;

const MEM_COMMIT: u32 = 0x0000_1000;
const MEM_IMAGE: u32 = 0x0100_0000;
const PAGE_NOACCESS: u32 = 0x01;
const PAGE_EXECUTE: u32 = 0x10;
const PAGE_GUARD: u32 = 0x100;

const OFN_FILEMUSTEXIST: u32 = 0x0000_1000;
const OFN_PATHMUSTEXIST: u32 = 0x0000_0800;
const OFN_EXPLORER: u32 = 0x0008_0000;
const OFN_NOCHANGEDIR: u32 = 0x0000_0008;

const ID_EDIT: i32 = 101;
const ID_BROWSE: i32 = 102;
const ID_PLAY: i32 = 103;
const ID_SCALE: i32 = 108;
const TIMER_TICK: usize = 1;
const EN_CHANGE: u32 = 0x0300;
const EN_KILLFOCUS: u32 = 0x0200;

const DWMWA_WINDOW_CORNER_PREFERENCE: u32 = 33;
const DWMWCP_ROUND: u32 = 2;
const DWMWA_CAPTION_COLOR: u32 = 35;
const DWMWA_TEXT_COLOR: u32 = 36;

static CLASS_NAME: &[u16] = &[
    b'f' as u16, b'p' as u16, b's' as u16, b'l' as u16, b'a' as u16, b'u' as u16, b'n' as u16,
    b'c' as u16, b'h' as u16, 0,
];

static mut EDIT: Hwnd = ptr::null_mut();
static mut EDIT_SCALE: Hwnd = ptr::null_mut();
static mut BTN_BROWSE: Hwnd = ptr::null_mut();
static mut BTN_PLAY: Hwnd = ptr::null_mut();
static mut MAIN: Hwnd = ptr::null_mut();
static mut SLIDER: Rect = Rect { left: 0, top: 0, right: 0, bottom: 0 };
static mut DRAGGING: bool = false;
static mut FIELD_BRUSH: Hbrush = ptr::null_mut();
static mut SYNCING: bool = false;
static mut PAGE: usize = 0;
static mut TILES: Vec<(Rect, usize)> = Vec::new();
static mut ARROWS: Option<(Rect, Rect)> = None;

const fn rgb(r: u32, g: u32, b: u32) -> u32 {
    (b << 16) | (g << 8) | r
}

const BG: u32 = rgb(0x0C, 0x0D, 0x11);
const CARD: u32 = rgb(0x16, 0x19, 0x20);
const FIELD: u32 = rgb(0x0D, 0x0F, 0x14);
const BORDER: u32 = rgb(0x2A, 0x2F, 0x3B);
const HILITE: u32 = rgb(0x1F, 0x24, 0x2D);
const TEXT: u32 = rgb(0xF0, 0xF2, 0xF7);
const MUTED: u32 = rgb(0x8A, 0x91, 0xA0);
const ACCENT: u32 = rgb(0x6C, 0x8C, 0xFF);
const ACCENT_DARK: u32 = rgb(0x4F, 0x6E, 0xE0);
const TRACK: u32 = rgb(0x2A, 0x2F, 0x3B);
const DANGER: u32 = rgb(0xFF, 0x6B, 0x6B);

static SCALE_BITS: AtomicU64 = AtomicU64::new(0);
static USE_INI: AtomicBool = AtomicBool::new(false);
static STATUS: Mutex<String> = Mutex::new(String::new());
static GAMES: Mutex<Vec<String>> = Mutex::new(Vec::new());
static ICONS: Mutex<Vec<(String, isize)>> = Mutex::new(Vec::new());
static GAME_PIDS: Mutex<Vec<u32>> = Mutex::new(Vec::new());

static mut F_TITLE: Hfont = ptr::null_mut();
static mut F_HEAD: Hfont = ptr::null_mut();
static mut F_BODY: Hfont = ptr::null_mut();
static mut F_SMALL: Hfont = ptr::null_mut();
static mut F_BIG: Hfont = ptr::null_mut();
static mut F_VALUE: Hfont = ptr::null_mut();

const BASE_CONST: u32 = 10_000_000;
const DEFAULT_SCALE: f64 = 6.0;

const PRE64: [u8; 3] = [0x48, 0x69, 0xC1];
const POST64: [u8; 7] = [0x8B, 0xCA, 0x33, 0xD2, 0x48, 0xF7, 0xF1];
const PRE32: [u8; 1] = [0xBB];
const POST32: [u8; 9] = [0x85, 0xF6, 0x74, 0x11, 0x8D, 0x41, 0x01, 0xF7, 0xE3];

fn wide(s: &str) -> Vec<u16> {
    s.encode_utf16().chain(std::iter::once(0)).collect()
}

fn exe_dir() -> PathBuf {
    std::env::current_exe()
        .ok()
        .and_then(|p| p.parent().map(Path::to_path_buf))
        .unwrap_or_else(|| PathBuf::from("."))
}

fn sibling_exe() -> &'static str {
    if cfg!(target_pointer_width = "64") {
        "MFG32.exe"
    } else {
        "MFG.exe"
    }
}

fn pe_machine(path: &Path) -> u16 {
    let Ok(d) = std::fs::read(path) else { return 0 };
    if d.len() < 0x40 || &d[..2] != b"MZ" {
        return 0;
    }
    let lfanew = u32::from_le_bytes([d[0x3C], d[0x3D], d[0x3E], d[0x3F]]) as usize;
    if lfanew + 6 > d.len() || &d[lfanew..lfanew + 2] != b"PE" {
        return 0;
    }
    u16::from_le_bytes([d[lfanew + 4], d[lfanew + 5]])
}

fn game_is64(path: &Path) -> Option<bool> {
    match pe_machine(path) {
        0x8664 => Some(true),
        0x14C => Some(false),
        _ => None,
    }
}

fn plain_path(p: &Path) -> PathBuf {
    let s = p.to_string_lossy().replace('/', "\\");
    if let Some(rest) = s.strip_prefix(r"\\?\UNC\") {
        return PathBuf::from(format!(r"\\{rest}"));
    }
    PathBuf::from(s.trim_start_matches(r"\\?\").to_string())
}

fn ini_path() -> PathBuf {
    exe_dir().join("fps.ini")
}

fn ini_value(key: &str) -> Option<String> {
    let text = std::fs::read_to_string(ini_path()).ok()?;
    for line in text.lines() {
        let l = line.trim();
        if l.is_empty() || l.starts_with('#') || l.starts_with(';') {
            continue;
        }
        if let Some((k, v)) = l.split_once('=') {
            if k.trim().eq_ignore_ascii_case(key) {
                return Some(v.trim().to_string());
            }
        }
    }
    None
}

fn read_scale() -> f64 {
    ini_value("scale")
        .and_then(|v| v.parse::<f64>().ok())
        .filter(|x| *x > 0.0)
        .unwrap_or(DEFAULT_SCALE)
}

fn ensure_minimal_ini() {
    let p = ini_path();
    if p.exists() {
        return;
    }
    let _ = std::fs::write(&p, "scale=6\r\n");
}

fn scale() -> f64 {
    let bits = SCALE_BITS.load(Ordering::Relaxed);
    if bits == 0 {
        DEFAULT_SCALE
    } else {
        f64::from_bits(bits)
    }
}

fn set_scale(v: f64) {
    SCALE_BITS.store(v.to_bits(), Ordering::Relaxed);
}

const REG_SUB: &str = "Software\\MFG";
const HKEY_CURRENT_USER: usize = 0x8000_0001;
const KEY_READ: u32 = 0x2_0019;
const KEY_WRITE: u32 = 0x2_0006;

unsafe fn reg_open(write: bool) -> Option<usize> {
    let sub = wide(REG_SUB);
    let mut key: usize = 0;
    let mut disp: u32 = 0;
    let rc = RegCreateKeyExW(
        HKEY_CURRENT_USER,
        sub.as_ptr(),
        0,
        ptr::null(),
        0,
        if write { KEY_WRITE } else { KEY_READ },
        ptr::null_mut(),
        &mut key,
        &mut disp,
    );
    if rc != 0 {
        None
    } else {
        Some(key)
    }
}

fn reg_get(name: &str) -> String {
    unsafe {
        let Some(key) = reg_open(false) else {
            return String::new();
        };
        let n = wide(name);
        let mut buf = vec![0u8; 4096];
        let mut len = buf.len() as u32;
        let mut kind: u32 = 0;
        let rc = RegQueryValueExW(key, n.as_ptr(), ptr::null_mut(), &mut kind, buf.as_mut_ptr(), &mut len);
        RegCloseKey(key);
        if rc != 0 || len < 2 {
            return String::new();
        }
        let u16s: Vec<u16> = buf[..len as usize]
            .chunks_exact(2)
            .map(|c| u16::from_le_bytes([c[0], c[1]]))
            .collect();
        String::from_utf16_lossy(&u16s).trim_end_matches('\0').to_string()
    }
}

fn reg_set(name: &str, value: &str) {
    unsafe {
        let Some(key) = reg_open(true) else { return };
        let n = wide(name);
        let v = wide(value);
        RegSetValueExW(key, n.as_ptr(), 0, 1, v.as_ptr() as *const u8, (v.len() * 2) as u32);
        RegCloseKey(key);
    }
}

fn scan(buf: &[u8], pre: &[u8], post: &[u8]) -> Vec<usize> {
    let mut out = Vec::new();
    let head = pre.len();
    if buf.len() < head + 4 + post.len() {
        return out;
    }
    for i in 0..buf.len() - (head + 4 + post.len()) {
        if &buf[i..i + head] == pre && &buf[i + head + 4..i + head + 4 + post.len()] == post {
            out.push(i);
        }
    }
    out
}

fn collect_hits(h: Handle, base: usize, size: usize, pre: &[u8], post: &[u8], out: &mut Vec<usize>) {
    let span = pre.len() + 4 + post.len();
    let chunk = 1usize << 20;
    let mut off = 0usize;
    let mut carry: Vec<u8> = Vec::new();
    while off < size {
        let n = chunk.min(size - off);
        let mut tmp = vec![0u8; n];
        let mut got: usize = 0;
        let ok = unsafe {
            ReadProcessMemory(h, (base + off) as *const _, tmp.as_mut_ptr() as *mut _, n, &mut got)
        };
        if ok != 0 && got != 0 {
            let skip = carry.len();
            let mut buf = Vec::with_capacity(skip + got);
            buf.extend_from_slice(&carry);
            buf.extend_from_slice(&tmp[..got]);
            for i in scan(&buf, pre, post) {
                out.push(base + off - skip + i + pre.len());
                if out.len() > 1 {
                    return;
                }
            }
            let keep = span.saturating_sub(1).min(buf.len());
            carry = buf[buf.len() - keep..].to_vec();
        } else {
            carry.clear();
        }
        off += n;
    }
}

fn find_site(h: Handle) -> Option<usize> {
    let mut images: Vec<(usize, usize)> = Vec::new();
    unsafe {
        let mut addr = 0usize;
        let mut mi: MemInfo = zeroed();
        for _ in 0..100_000 {
            let n = VirtualQueryEx(h, addr as *const _, &mut mi, size_of::<MemInfo>());
            if n == 0 || mi.region_size == 0 {
                break;
            }
            if mi.state == MEM_COMMIT
                && mi.mem_type == MEM_IMAGE
                && mi.protect & (PAGE_NOACCESS | PAGE_GUARD | PAGE_EXECUTE) == 0
            {
                images.push((mi.base, mi.region_size));
            }
            let next = mi.base.wrapping_add(mi.region_size);
            if next <= addr {
                break;
            }
            addr = next;
        }
    }

    for (pre, post) in [(&PRE64[..], &POST64[..]), (&PRE32[..], &POST32[..])] {
        let mut hits: Vec<usize> = Vec::new();
        for &(base, size) in &images {
            collect_hits(h, base, size, pre, post, &mut hits);
            if hits.len() > 1 {
                break;
            }
        }
        if hits.len() == 1 {
            return Some(hits[0]);
        }
    }
    None
}

fn write_const(h: Handle, addr: usize, value: u32) -> bool {
    unsafe {
        let mut old: u32 = 0;
        if VirtualProtectEx(h, addr as *mut _, 4, PAGE_EXECUTE_READWRITE, &mut old) == 0 {
            return false;
        }
        let mut written: usize = 0;
        let ok = WriteProcessMemory(
            h,
            addr as *mut _,
            &value as *const u32 as *const _,
            4,
            &mut written,
        );
        let mut back: u32 = 0;
        VirtualProtectEx(h, addr as *mut _, 4, old, &mut back);
        FlushInstructionCache(h, addr as *const _, 4);
        ok != 0 && written == 4
    }
}

fn pid_alive(pid: u32) -> bool {
    process_list().iter().any(|&(p, _)| p == pid)
}

fn patch_loop(pid: u32) {
    let mut h: Handle = ptr::null_mut();
    let mut site = 0usize;
    let mut want = ((BASE_CONST as f64) * scale()).round() as u32;
    let mut tick = 0u32;

    loop {
        if h.is_null() {
            h = unsafe { OpenProcess(PROCESS_PATCH, 0, pid) };
            if h.is_null() {
                if !pid_alive(pid) {
                    return;
                }
                std::thread::sleep(std::time::Duration::from_millis(300));
                continue;
            }
        } else if unsafe { WaitForSingleObject(h, 0) } == 0 {
            unsafe { CloseHandle(h) };
            return;
        }

        if tick % 8 == 0 {
            let s = if USE_INI.load(Ordering::Relaxed) {
                read_scale()
            } else {
                scale()
            };
            want = ((BASE_CONST as f64) * s).round() as u32;
        }

        if site == 0 && tick % 5 == 0 {
            site = find_site(h).unwrap_or(0);
        }

        if site != 0 {
            let mut cur: u32 = 0;
            let mut got: usize = 0;
            let ok = unsafe {
                ReadProcessMemory(h, site as *const _, &mut cur as *mut u32 as *mut _, 4, &mut got)
            };
            if ok == 0 || got != 4 {
                site = 0;
            } else if cur != want {
                write_const(h, site, want);
            }
        }

        tick += 1;
        std::thread::sleep(std::time::Duration::from_millis(400));
    }
}

fn patch_into(pid: u32, exe: &Path) {
    let is64 = match pe_machine(exe) {
        0x8664 => true,
        0x14C => false,
        _ => return,
    };
    if cfg!(target_pointer_width = "64") || !is64 {
        std::thread::spawn(move || patch_loop(pid));
        return;
    }
    let sib = exe_dir().join(sibling_exe());
    if !sib.exists() {
        return;
    }
    let cmd = format!("\"{}\" --patch {pid}", sib.display());
    unsafe {
        let mut si: StartupInfoW = zeroed();
        si.cb = size_of::<StartupInfoW>() as u32;
        let mut pi: ProcessInformation = zeroed();
        if CreateProcessW(
            wide(&sib.to_string_lossy()).as_ptr(),
            wide(&cmd).as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
            ptr::null(),
            &mut si,
            &mut pi,
        ) != 0
        {
            CloseHandle(pi.h_thread);
            CloseHandle(pi.h_process);
        }
    }
}

const FILE_MAP_ALL_ACCESS: u32 = 0xF001F;
const SEC_NAME: &str = "RTSSSharedMemoryV2";
const SIG_RTSS: u32 = 0x5254_5353;

const H_SIG: usize = 0x00;
const H_VER: usize = 0x04;
const H_OSD_ENTRY_SIZE: usize = 0x14;
const H_OSD_OFF: usize = 0x18;
const H_OSD_SIZE: usize = 0x1C;
const H_OSD_FRAME: usize = 0x20;
const H_BUSY: usize = 0x24;

const O_TEXT: usize = 0x000;
const O_OWNER: usize = 0x100;
const O_TEXT_EX: usize = 0x200;
const O_TEXT_EX2: usize = 0x41200;

const OSD_OWNER: &str = "fps_shadow";

struct Smem {
    h: Handle,
    base: *mut u8,
}

impl Smem {
    unsafe fn open() -> Option<Smem> {
        let name = wide(SEC_NAME);
        let h = OpenFileMappingW(FILE_MAP_ALL_ACCESS, 0, name.as_ptr());
        if h.is_null() {
            return None;
        }
        let p = MapViewOfFile(h, FILE_MAP_ALL_ACCESS, 0, 0, 0);
        if p.is_null() {
            CloseHandle(h);
            return None;
        }
        let s = Smem { h, base: p as *mut u8 };
        if s.u32(H_SIG) != SIG_RTSS {
            s.close();
            return None;
        }
        Some(s)
    }

    unsafe fn close(self) {
        UnmapViewOfFile(self.base as *const core::ffi::c_void);
        CloseHandle(self.h);
    }

    unsafe fn u32(&self, off: usize) -> u32 {
        ptr::read_unaligned(self.base.add(off) as *const u32)
    }

    unsafe fn put_u32(&self, off: usize, v: u32) {
        ptr::write_unaligned(self.base.add(off) as *mut u32, v);
    }

    unsafe fn cstr(&self, off: usize, max: usize) -> String {
        let buf = std::slice::from_raw_parts(self.base.add(off), max);
        let end = buf.iter().position(|&b| b == 0).unwrap_or(max);
        String::from_utf8_lossy(&buf[..end]).into_owned()
    }

    unsafe fn put_str(&self, off: usize, size: usize, text: &str) {
        let bytes = text.as_bytes();
        let n = bytes.len().min(size.saturating_sub(1));
        ptr::write_bytes(self.base.add(off), 0, size);
        if n > 0 {
            ptr::copy_nonoverlapping(bytes.as_ptr(), self.base.add(off), n);
        }
    }

    unsafe fn bump_frame(&self) {
        self.put_u32(H_OSD_FRAME, self.u32(H_OSD_FRAME).wrapping_add(1));
    }

    unsafe fn slot_base(&self, i: usize) -> Option<usize> {
        let off = self.u32(H_OSD_OFF) as usize;
        let size = self.u32(H_OSD_ENTRY_SIZE) as usize;
        let count = self.u32(H_OSD_SIZE) as usize;
        if i >= count || size < 0x1200 || size > 0x100000 {
            return None;
        }
        Some(off + i * size)
    }

    unsafe fn owner(&self, base: usize) -> String {
        self.cstr(base + O_OWNER, 256)
    }

    unsafe fn acquire_slot(&self) -> Option<usize> {
        let count = self.u32(H_OSD_SIZE) as usize;
        let mut first_free = None;
        for i in 0..count.min(64) {
            let Some(base) = self.slot_base(i) else { continue };
            let o = self.owner(base);
            if o == OSD_OWNER {
                return Some(base);
            }
            if o.is_empty() && i >= 2 && first_free.is_none() {
                first_free = Some(base);
            }
        }
        first_free
    }

    unsafe fn clear_slot(&self, base: usize) {
        self.put_str(base + O_TEXT, 256, "");
        self.put_str(base + O_TEXT_EX, 4096, "");
        self.put_str(base + O_TEXT_EX2, 32768, "");
        self.put_str(base + O_OWNER, 256, "");
        self.bump_frame();
    }
}

fn osd_clear() {
    unsafe {
        let Some(s) = Smem::open() else { return };
        let count = s.u32(H_OSD_SIZE) as usize;
        for i in 0..count.min(64) {
            let Some(base) = s.slot_base(i) else { continue };
            if s.owner(base) == OSD_OWNER {
                s.clear_slot(base);
            }
        }
        s.close();
    }
}

fn game_running() -> bool {
    let pids = GAME_PIDS.lock().map(|v| v.clone()).unwrap_or_default();
    !pids.is_empty() && pids.iter().any(|&p| pid_alive(p))
}

fn games() -> Vec<String> {
    GAMES.lock().map(|g| g.clone()).unwrap_or_default()
}

fn set_games(list: Vec<String>) {
    if let Ok(mut g) = GAMES.lock() {
        *g = list;
    }
}

fn games_save() {
    reg_set("games", &games().join("\n"));
}

fn game_name(path: &str) -> String {
    Path::new(path)
        .file_stem()
        .map(|s| s.to_string_lossy().to_string())
        .unwrap_or_else(|| path.to_string())
}

unsafe fn games_load() {
    let saved = reg_get("games");
    let mut list: Vec<String> = saved
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();
    let last = reg_get("exe");
    if list.is_empty() && !last.is_empty() {
        list.push(last);
    }
    list.dedup();
    set_games(list);
}

unsafe fn add_game(owner: Hwnd) {
    let Some(p) = browse(owner) else { return };
    let text = p.to_string_lossy().to_string();
    let mut list = games();
    if !list.iter().any(|g| plain(Path::new(g)) == plain(&p)) {
        list.push(text.clone());
        set_games(list);
        games_save();
    }
    set_text(EDIT, &text);
    PAGE = 0;
    set_status("");
    save_ui();
    InvalidateRect(MAIN, ptr::null(), 0);
}

unsafe fn remove_game(i: usize) {
    let mut list = games();
    if i >= list.len() {
        return;
    }
    let gone = list.remove(i);
    set_games(list);
    games_save();
    if plain(Path::new(&gone)) == plain(Path::new(&get_text(EDIT))) {
        set_text(EDIT, "");
    }
    if PAGE > 0 && PAGE >= pages() {
        PAGE -= 1;
    }
    InvalidateRect(MAIN, ptr::null(), 0);
}

fn ui_value(reg: &str, ini: &str) -> String {
    let v = reg_get(reg);
    if v.is_empty() {
        ini_value(ini).unwrap_or_default()
    } else {
        v
    }
}

unsafe fn load_ui() {
    set_text(EDIT, &ui_value("exe", "exe"));
    let stored = reg_get("scale").parse::<f64>().ok();
    set_scale(stored.unwrap_or_else(read_scale).clamp(1.0, 10.0));
    sync_scale_text();
    games_load();
}

unsafe fn save_ui() {
    reg_set("exe", &get_text(EDIT));
    reg_set("scale", &format!("{:.1}", scale()));
}

fn process_list() -> Vec<(u32, u32)> {
    let mut out = Vec::new();
    unsafe {
        let snap = CreateToolhelp32Snapshot(TH32CS_SNAPPROCESS, 0);
        if snap.is_null() || snap as isize == -1 {
            return out;
        }
        let mut e: ProcessEntry32W = zeroed();
        e.dw_size = size_of::<ProcessEntry32W>() as u32;
        if Process32FirstW(snap, &mut e) != 0 {
            loop {
                out.push((e.th32_process_id, e.th32_parent_process_id));
                e.dw_size = size_of::<ProcessEntry32W>() as u32;
                if Process32NextW(snap, &mut e) == 0 {
                    break;
                }
            }
        }
        CloseHandle(snap);
    }
    out
}

fn image_path(pid: u32) -> Option<PathBuf> {
    unsafe {
        let h = OpenProcess(PROCESS_QUERY_LIMITED, 0, pid);
        if h.is_null() {
            return None;
        }
        let mut buf = [0u16; 1024];
        let mut n = buf.len() as u32;
        let ok = QueryFullProcessImageNameW(h, 0, buf.as_mut_ptr(), &mut n);
        CloseHandle(h);
        if ok == 0 {
            return None;
        }
        Some(PathBuf::from(String::from_utf16_lossy(&buf[..n as usize])))
    }
}

fn plain(p: &Path) -> String {
    let s = p.to_string_lossy().replace('/', "\\");
    s.trim_start_matches(r"\\?\").to_lowercase()
}

fn track_pid(pid: u32) {
    if let Ok(mut v) = GAME_PIDS.lock() {
        if !v.contains(&pid) {
            v.push(pid);
        }
    }
}

fn watch_children(root: u32, game_dir: &Path) {
    let root_dir = plain(game_dir);
    let mut seen: Vec<u32> = vec![root];
    let mut done: Vec<u32> = vec![root];
    let mut quiet = 0u32;
    for _ in 0..900 {
        std::thread::sleep(std::time::Duration::from_millis(300));
        let list = process_list();
        if list.is_empty() {
            break;
        }
        let mut fresh: Vec<u32> = Vec::new();
        for &(pid, ppid) in &list {
            if pid != root && seen.contains(&ppid) && !seen.contains(&pid) {
                fresh.push(pid);
            }
        }
        for pid in fresh {
            seen.push(pid);
            let Some(path) = image_path(pid) else { continue };
            if !plain(&path).starts_with(&root_dir) {
                continue;
            }
            track_pid(pid);
            patch_into(pid, &path);
            done.push(pid);
        }
        let alive = list.iter().any(|&(pid, _)| done.contains(&pid));
        if alive {
            quiet = 0;
        } else {
            quiet += 1;
            if quiet > 20 {
                break;
            }
        }
    }
}

fn launch_game(game: &Path) -> Result<(u32, PathBuf), String> {
    ensure_minimal_ini();
    let dir = game.parent().unwrap_or(Path::new("."));
    let cmd = format!("\"{}\"", game.display());
    let mut si: StartupInfoW = unsafe { zeroed() };
    si.cb = size_of::<StartupInfoW>() as u32;
    let mut pi: ProcessInformation = unsafe { zeroed() };

    unsafe {
        let ok = CreateProcessW(
            wide(&game.to_string_lossy()).as_ptr(),
            wide(&cmd).as_ptr(),
            ptr::null_mut(),
            ptr::null_mut(),
            0,
            0,
            ptr::null_mut(),
            wide(&dir.to_string_lossy()).as_ptr(),
            &mut si,
            &mut pi,
        );
        if ok == 0 {
            return Err(format!(
                "could not start the game ({})",
                std::io::Error::last_os_error()
            ));
        }
        CloseHandle(pi.h_thread);
        CloseHandle(pi.h_process);
    }
    let pid = pi.process_id;
    track_pid(pid);
    patch_into(pid, game);
    Ok((pid, dir.to_path_buf()))
}

fn play(game: &Path) -> Result<(String, Option<(u32, PathBuf)>), String> {
    let game = &plain_path(&std::fs::canonicalize(game).unwrap_or_else(|_| game.to_path_buf()));
    let Some(is64) = game_is64(game) else {
        return Err(format!("{} is not a Windows .exe", game.display()));
    };
    let (pid, dir) = launch_game(game)?;
    Ok((
        format!(
            "running  •  pid {pid}  •  {} game  •  scale ×{:.1}",
            if is64 { "64-bit" } else { "32-bit" },
            scale()
        ),
        Some((pid, dir)),
    ))
}

fn fill(hdc: Hdc, r: Rect, color: u32) {
    unsafe {
        let b = CreateSolidBrush(color);
        FillRect(hdc, &r, b);
        DeleteObject(b);
    }
}

fn round_fill(hdc: Hdc, r: Rect, color: u32, radius: i32) {
    unsafe {
        let b = CreateSolidBrush(color);
        let old_b = SelectObject(hdc, b);
        let old_p = SelectObject(hdc, GetStockObject(NULL_PEN));
        RoundRect(hdc, r.left, r.top, r.right, r.bottom, radius, radius);
        SelectObject(hdc, old_b);
        SelectObject(hdc, old_p);
        DeleteObject(b);
    }
}

fn boxed(hdc: Hdc, r: Rect, border: u32, inner: u32, radius: i32) {
    round_fill(hdc, r, border, radius);
    round_fill(
        hdc,
        Rect { left: r.left + 1, top: r.top + 1, right: r.right - 1, bottom: r.bottom - 1 },
        inner,
        (radius - 1).max(1),
    );
}

fn card(hdc: Hdc, r: Rect) {
    boxed(hdc, r, BORDER, CARD, 16);
    fill(
        hdc,
        Rect { left: r.left + 14, top: r.top + 1, right: r.right - 14, bottom: r.top + 2 },
        HILITE,
    );
}

fn dot(hdc: Hdc, cx: i32, cy: i32, color: u32) {
    unsafe {
        let b = CreateSolidBrush(color);
        let old_b = SelectObject(hdc, b);
        let old_p = SelectObject(hdc, GetStockObject(NULL_PEN));
        Ellipse(hdc, cx - 4, cy - 4, cx + 4, cy + 4);
        SelectObject(hdc, old_b);
        SelectObject(hdc, old_p);
        DeleteObject(b);
    }
}

unsafe fn section_label(hdc: Hdc, x: i32, y: i32, s: &str) {
    dot(hdc, x - 11, y + 9, ACCENT);
    text_at(
        hdc,
        F_HEAD,
        TEXT,
        s,
        Rect { left: x, top: y, right: x + 320, bottom: y + 18 },
        DT_LEFT,
    );
}

fn text_at(hdc: Hdc, font: Hfont, color: u32, s: &str, r: Rect, flags: u32) {
    unsafe {
        let old = SelectObject(hdc, font);
        SetTextColor(hdc, color);
        SetBkMode(hdc, 1);
        let w = wide(s);
        let mut rr = Rect { left: r.left, top: r.top, right: r.right, bottom: r.bottom };
        DrawTextW(hdc, w.as_ptr(), -1, &mut rr, flags | DT_SINGLELINE | DT_VCENTER);
        SelectObject(hdc, old);
    }
}

fn slider_pos() -> f32 {
    (((scale() - 1.0) * 10.0).round() as i32).clamp(0, 90) as f32
}

unsafe fn set_scale_from_x(x: i32) {
    let (l, r) = (SLIDER.left, SLIDER.right);
    if r <= l {
        return;
    }
    let t = ((x - l) as f32 / (r - l) as f32).clamp(0.0, 1.0);
    let step = (t * 90.0).round();
    let v = 1.0 + step as f64 * 0.1;
    set_scale(v);
    sync_scale_text();
    InvalidateRect(MAIN, ptr::null(), 0);
}

unsafe fn knob_x(steps: f32) -> i32 {
    let r = SLIDER;
    let pad = 9;
    let l = r.left + pad;
    let rr = r.right - pad;
    l + ((rr - l) as f32 * (steps / 90.0)) as i32
}

unsafe fn draw_slider(hdc: Hdc) {
    let r = SLIDER;
    let cy = (r.top + r.bottom) / 2;
    let track = Rect { left: r.left, top: cy - 3, right: r.right, bottom: cy + 3 };
    round_fill(hdc, track, TRACK, 6);

    let steps = slider_pos();
    let kx = knob_x(steps);
    let done = Rect { left: r.left, top: cy - 3, right: kx.max(r.left + 4), bottom: cy + 3 };
    round_fill(hdc, done, ACCENT, 6);

    for i in (0..=90).step_by(5) {
        let x = knob_x(i as f32);
        let on = (i as f32) <= steps;
        let d = Rect { left: x - 1, top: cy - 1, right: x + 1, bottom: cy + 1 };
        fill(hdc, d, if on { BG } else { BORDER });
    }

    let ring = Rect { left: kx - 11, top: cy - 11, right: kx + 11, bottom: cy + 11 };
    let b = CreateSolidBrush(BG);
    let old_b = SelectObject(hdc, b);
    let old_p = SelectObject(hdc, GetStockObject(NULL_PEN));
    let ring_b = CreateSolidBrush(ACCENT);
    SelectObject(hdc, ring_b);
    Ellipse(hdc, ring.left, ring.top, ring.right, ring.bottom);
    SelectObject(hdc, b);
    Ellipse(hdc, kx - 7, cy - 7, kx + 7, cy + 7);
    SelectObject(hdc, old_b);
    SelectObject(hdc, old_p);
    DeleteObject(b);
    DeleteObject(ring_b);

    for i in 0..10 {
        let x = knob_x((i * 10) as f32);
        let lr = Rect { left: x - 14, top: r.bottom + 4, right: x + 14, bottom: r.bottom + 20 };
        text_at(hdc, F_SMALL, MUTED, &format!("{}", i + 1), lr, DT_CENTER);
    }
}

const TILE_W: i32 = 92;
const TILE_H: i32 = 66;
const TILE_COLS: usize = 6;
const TILE_ROWS: usize = 2;
const PER_PAGE: usize = TILE_COLS * TILE_ROWS;
const GRID_TOP: i32 = 412;
const DI_NORMAL: u32 = 0x0003;

unsafe fn pages() -> usize {
    let n = games().len();
    (n + PER_PAGE - 1) / PER_PAGE
}

fn game_buttons() -> (Rect, Rect, Rect) {
    let y = 366;
    (
        Rect { left: 586, top: y, right: 608, bottom: y + 22 },
        Rect { left: 614, top: y, right: 632, bottom: y + 22 },
        Rect { left: 636, top: y, right: 654, bottom: y + 22 },
    )
}

fn in_rect(r: Rect, x: i32, y: i32) -> bool {
    x >= r.left && x < r.right && y >= r.top && y < r.bottom
}

unsafe fn layout_tiles() -> Vec<(Rect, usize)> {
    let n = games().len();
    let start = PAGE * PER_PAGE;
    let mut out = Vec::new();
    for (slot, i) in (start..n.min(start + PER_PAGE)).enumerate() {
        let l = 40 + (slot % TILE_COLS) as i32 * (TILE_W + 6);
        let t = GRID_TOP + (slot / TILE_COLS) as i32 * TILE_H;
        out.push((
            Rect { left: l, top: t, right: l + TILE_W, bottom: t + TILE_H - 6 },
            i,
        ));
    }
    out
}

unsafe fn icon_for(path: &str) -> isize {
    if let Ok(c) = ICONS.lock() {
        if let Some((_, h)) = c.iter().find(|(p, _)| p == path) {
            return *h;
        }
    }
    let mut large: Handle = ptr::null_mut();
    let mut small: Handle = ptr::null_mut();
    let n = ExtractIconExW(wide(path).as_ptr(), 0, &mut large, &mut small, 1);
    if !small.is_null() {
        DestroyIcon(small);
    }
    let h = if n == 0 || large.is_null() { 0 } else { large as isize };
    if let Ok(mut c) = ICONS.lock() {
        c.push((path.to_string(), h));
    }
    h
}

unsafe fn draw_tile(hdc: Hdc, r: Rect, path: &str, selected: bool) {
    if selected {
        boxed(hdc, r, ACCENT, rgb(0x14, 0x1B, 0x30), 14);
    } else {
        boxed(hdc, r, BORDER, FIELD, 14);
    }

    let cx = (r.left + r.right) / 2;
    let iy = r.top + 5;
    let icon = icon_for(path);
    if icon != 0 {
        DrawIconEx(
            hdc,
            cx - 16,
            iy,
            icon as Handle,
            32,
            32,
            0,
            ptr::null_mut(),
            0,
            DI_NORMAL,
        );
    } else {
        let boxr = Rect { left: cx - 14, top: iy + 2, right: cx + 14, bottom: iy + 30 };
        round_fill(hdc, boxr, TRACK, 8);
        let letter = game_name(path)
            .chars()
            .next()
            .map(|c| c.to_uppercase().to_string())
            .unwrap_or_default();
        text_at(
            hdc,
            F_HEAD,
            if selected { ACCENT } else { MUTED },
            &letter,
            boxr,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );
    }

    text_at(
        hdc,
        F_SMALL,
        if selected { TEXT } else { MUTED },
        &game_name(path),
        Rect { left: r.left + 4, top: r.top + 40, right: r.right - 4, bottom: r.top + 56 },
        DT_CENTER | DT_SINGLELINE | DT_END_ELLIPSIS,
    );
}

unsafe fn draw_hbtn(hdc: Hdc, r: Rect, glyph: &str, live: bool) {
    boxed(hdc, r, if live { BORDER } else { HILITE }, FIELD, 7);
    text_at(
        hdc,
        F_BODY,
        if live { TEXT } else { TRACK },
        glyph,
        r,
        DT_CENTER | DT_VCENTER | DT_SINGLELINE,
    );
}

unsafe fn draw_games(hdc: Hdc) {
    let current = plain(Path::new(&get_text(EDIT)));
    let rects = layout_tiles();
    for &(r, i) in &rects {
        let list = games();
        let Some(p) = list.get(i) else { continue };
        let selected = !current.is_empty() && plain(Path::new(p)) == current;
        draw_tile(hdc, r, p, selected);
    }
    TILES = rects;

    let (plus, prev, next) = game_buttons();
    let total = pages();
    let multi = total > 1;
    draw_hbtn(hdc, plus, "+", true);
    draw_hbtn(hdc, prev, "‹", multi && PAGE > 0);
    draw_hbtn(hdc, next, "›", multi && PAGE + 1 < total);
    if multi {
        text_at(
            hdc,
            F_SMALL,
            MUTED,
            &format!("{} / {}", PAGE + 1, total),
            Rect { left: 536, top: 370, right: 580, bottom: 386 },
            DT_RIGHT,
        );
    }
    ARROWS = if multi { Some((prev, next)) } else { None };

    if games().is_empty() {
        text_at(
            hdc,
            F_SMALL,
            MUTED,
            "no games yet - press + and pick a .exe",
            Rect { left: 40, top: 440, right: 640, bottom: 458 },
            DT_CENTER,
        );
    }
}

unsafe fn paint(hwnd: Hwnd) {
    let mut ps: PaintStruct = zeroed();
    let hdc = BeginPaint(hwnd, &mut ps);
    let mut rc: Rect = zeroed();
    GetClientRect(hwnd, &mut rc);
    let w = rc.right;
    let h = rc.bottom;

    let mem = CreateCompatibleDC(hdc);
    let bmp = CreateCompatibleBitmap(hdc, w, h);
    let old = SelectObject(mem, bmp);

    bg_gradient(mem, w, h);

    round_fill(mem, Rect { left: 28, top: 24, right: 68, bottom: 64 }, ACCENT, 11);
    text_at(
        mem,
        F_TITLE,
        BG,
        "M",
        Rect { left: 28, top: 24, right: 68, bottom: 64 },
        DT_CENTER | DT_VCENTER | DT_SINGLELINE,
    );
    text_at(mem, F_TITLE, TEXT, "MFG", Rect { left: 82, top: 18, right: 460, bottom: 46 }, DT_LEFT);
    text_at(
        mem,
        F_SMALL,
        MUTED,
        "Frame-rate scaler for RTSS  ·  real FPS × scale",
        Rect { left: 84, top: 46, right: 656, bottom: 64 },
        DT_LEFT,
    );
    fill(mem, Rect { left: 24, top: 84, right: 656, bottom: 85 }, BORDER);

    card(mem, Rect { left: 24, top: 98, right: 656, bottom: 196 });
    section_label(mem, 48, 112, "Game executable");
    text_at(
        mem,
        F_SMALL,
        MUTED,
        "the .exe you normally launch",
        Rect { left: 300, top: 113, right: 632, bottom: 129 },
        DT_RIGHT,
    );
    boxed(mem, Rect { left: 40, top: 146, right: 536, bottom: 182 }, BORDER, FIELD, 10);

    card(mem, Rect { left: 24, top: 212, right: 656, bottom: 340 });
    section_label(mem, 48, 226, "FPS scale");
    text_at(
        mem,
        F_SMALL,
        MUTED,
        "drag the slider or type a value from 1.0 to 10.0",
        Rect { left: 48, top: 248, right: 470, bottom: 264 },
        DT_LEFT,
    );
    text_at(
        mem,
        F_BIG,
        ACCENT,
        "×",
        Rect { left: 442, top: 214, right: 486, bottom: 262 },
        DT_RIGHT | DT_VCENTER | DT_SINGLELINE,
    );
    boxed(mem, Rect { left: 492, top: 214, right: 644, bottom: 262 }, BORDER, FIELD, 12);

    card(mem, Rect { left: 24, top: 356, right: 656, bottom: 548 });
    section_label(mem, 48, 370, "My games");
    text_at(
        mem,
        F_SMALL,
        MUTED,
        "click select  ·  double-click play  ·  right-click remove",
        Rect { left: 48, top: 390, right: 540, bottom: 406 },
        DT_LEFT,
    );
    draw_games(mem);

    let status = STATUS.lock().map(|s| s.clone()).unwrap_or_default();
    let is_err = status.starts_with("error");
    let is_run = status.starts_with("running");
    let status_col = if is_err {
        DANGER
    } else if is_run {
        ACCENT
    } else {
        MUTED
    };
    if !status.is_empty() {
        dot(mem, 30, 567, status_col);
    }
    text_at(
        mem,
        F_BODY,
        if status.is_empty() { MUTED } else { status_col },
        &status,
        Rect { left: 42, top: 558, right: 440, bottom: 576 },
        DT_LEFT | DT_END_ELLIPSIS,
    );
    text_at(
        mem,
        F_SMALL,
        MUTED,
        "keep this window open until the game is in its menu",
        Rect { left: 26, top: 584, right: 440, bottom: 600 },
        DT_LEFT,
    );

    draw_slider(mem);

    BitBlt(hdc, 0, 0, w, h, mem, 0, 0, 0x00CC_0020);
    SelectObject(mem, old);
    DeleteObject(bmp);
    DeleteObject(mem);
    EndPaint(hwnd, &ps);
}

fn bg_gradient(hdc: Hdc, w: i32, h: i32) {
    let apply = |c: u32| ((c & 0xFF) << 8) as u16;
    let top = rgb(0x13, 0x15, 0x1C);
    let bottom = rgb(0x08, 0x09, 0x0C);
    let v = [
        TriVertex { x: 0, y: 0, red: apply(top), green: apply(top >> 8), blue: apply(top >> 16), alpha: 0 },
        TriVertex {
            x: 0,
            y: h,
            red: apply(bottom),
            green: apply(bottom >> 8),
            blue: apply(bottom >> 16),
            alpha: 0,
        },
    ];
    let r = [GradientRect { upper_left: 0, lower_right: 1 }];
    unsafe {
        if GradientFill(hdc, v.as_ptr(), 2, r.as_ptr(), 1, 1) == 0 {
            fill(hdc, Rect { left: 0, top: 0, right: w, bottom: h }, BG);
        }
    }
}

unsafe fn make_icon() -> Handle {
    let screen = GetDC(ptr::null_mut());
    if screen.is_null() {
        return ptr::null_mut();
    }
    let dc = CreateCompatibleDC(screen);
    let color = CreateCompatibleBitmap(screen, 32, 32);
    ReleaseDC(ptr::null_mut(), screen);
    let old = SelectObject(dc, color);

    let bg = CreateSolidBrush(BG);
    FillRect(dc, &Rect { left: 0, top: 0, right: 32, bottom: 32 }, bg);
    DeleteObject(bg);

    let acc = CreateSolidBrush(ACCENT);
    for &(y, w) in &[(7, 22), (14, 15), (21, 8)] {
        FillRect(
            dc,
            &Rect { left: 5, top: y, right: 5 + w, bottom: y + 5 },
            acc,
        );
    }
    DeleteObject(acc);

    SelectObject(dc, old);
    DeleteObject(dc);

    let bits = vec![0u8; 4 * 32];
    let mask = CreateBitmap(32, 32, 1, 1, bits.as_ptr() as *const _);
    let ii = IconInfo { f_icon: 1, x_hot: 0, y_hot: 0, h_mask: mask, h_color: color };
    let icon = CreateIconIndirect(&ii);
    DeleteObject(mask);
    DeleteObject(color);
    icon
}

fn get_text(h: Hwnd) -> String {
    let mut buf = [0u16; 1024];
    unsafe { GetWindowTextW(h, buf.as_mut_ptr(), buf.len() as i32) };
    let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    String::from_utf16_lossy(&buf[..n]).trim().to_string()
}

fn set_text(h: Hwnd, text: &str) {
    unsafe { SetWindowTextW(h, wide(text).as_ptr()) };
}

unsafe fn sync_scale_text() {
    if EDIT_SCALE.is_null() {
        return;
    }
    SYNCING = true;
    set_text(EDIT_SCALE, &format!("{:.1}", scale()));
    SYNCING = false;
}

unsafe fn apply_scale_text() {
    let typed = get_text(EDIT_SCALE).replace(',', ".");
    if let Ok(v) = typed.parse::<f64>() {
        if (1.0..=10.0).contains(&v) {
            set_scale((v * 10.0).round() / 10.0);
            InvalidateRect(MAIN, ptr::null(), 0);
        }
    }
}

fn set_status(text: &str) {
    if let Ok(mut s) = STATUS.lock() {
        *s = text.to_string();
    }
    unsafe { InvalidateRect(MAIN, ptr::null(), 0) };
}

unsafe fn make_controls(h: Hwnd) {
    let edit = wide("EDIT");
    let button = wide("BUTTON");

    let mk_edit = |id: i32, x: i32, y: i32, w: i32, hh: i32, text: &str| -> Hwnd {
        let e = CreateWindowExW(
            0,
            edit.as_ptr(),
            wide(text).as_ptr(),
            WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL,
            x,
            y,
            w,
            hh,
            h,
            id as Handle,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        SendMessageW(e, WM_SETFONT, F_BODY as usize, 1);
        e
    };

    let mk_button = |id: i32, x: i32, y: i32, w: i32, hh: i32, text: &str| -> Hwnd {
        let b = CreateWindowExW(
            0,
            button.as_ptr(),
            wide(text).as_ptr(),
            WS_CHILD | WS_VISIBLE | BS_OWNERDRAW,
            x,
            y,
            w,
            hh,
            h,
            id as Handle,
            ptr::null_mut(),
            ptr::null_mut(),
        );
        SendMessageW(b, WM_SETFONT, F_BODY as usize, 1);
        b
    };

    let game = ini_value("exe").unwrap_or_default();
    EDIT = mk_edit(ID_EDIT, 52, 153, 470, 22, &game);
    BTN_BROWSE = mk_button(ID_BROWSE, 548, 146, 108, 36, "Browse");

    BTN_PLAY = mk_button(ID_PLAY, 460, 556, 196, 40, "PLAY");

    EDIT_SCALE = CreateWindowExW(
        0,
        edit.as_ptr(),
        wide(&format!("{:.1}", scale())).as_ptr(),
        WS_CHILD | WS_VISIBLE | ES_AUTOHSCROLL | ES_CENTER,
        498,
        221,
        140,
        34,
        h,
        ID_SCALE as Handle,
        ptr::null_mut(),
        ptr::null_mut(),
    );
    SendMessageW(EDIT_SCALE, WM_SETFONT, F_VALUE as usize, 1);

    SLIDER = Rect { left: 48, top: 284, right: 632, bottom: 300 };
}

unsafe fn draw_button(dis: *const DrawItemStruct) {
    let d = &*dis;
    let r = d.rc_item;
    let pressed = d.item_state & ODS_SELECTED != 0;
    let play = d.ctl_id == ID_PLAY as u32;

    if play {
        let bg = if pressed { ACCENT_DARK } else { ACCENT };
        round_fill(d.hdc, r, bg, 12);
        text_at(
            d.hdc,
            F_HEAD,
            rgb(0xFF, 0xFF, 0xFF),
            "PLAY",
            r,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );
    } else {
        boxed(
            d.hdc,
            r,
            if pressed { ACCENT } else { BORDER },
            if pressed { rgb(0x1B, 0x20, 0x2B) } else { FIELD },
            9,
        );
        text_at(
            d.hdc,
            F_BODY,
            TEXT,
            "Browse",
            r,
            DT_CENTER | DT_VCENTER | DT_SINGLELINE,
        );
    }
}

unsafe fn tile_at(x: i32, y: i32) -> Option<usize> {
    TILES
        .iter()
        .find(|(r, _)| in_rect(*r, x, y))
        .map(|(_, i)| *i)
}

unsafe fn select_game(i: usize) {
    let list = games();
    let Some(p) = list.get(i) else { return };
    let p = p.clone();
    set_text(EDIT, &p);
    set_status("");
    save_ui();
    InvalidateRect(MAIN, ptr::null(), 0);
}

unsafe fn turn_page(delta: i32) {
    let total = pages() as i32;
    if total <= 1 {
        return;
    }
    PAGE = (PAGE as i32 + delta).clamp(0, total - 1) as usize;
    InvalidateRect(MAIN, ptr::null(), 0);
}

unsafe fn header_click(h: Hwnd, x: i32, y: i32) {
    let (plus, _prev, _next) = game_buttons();
    if in_rect(plus, x, y) {
        add_game(h);
    } else if let Some((p, n)) = ARROWS {
        if in_rect(p, x, y) {
            turn_page(-1);
        } else if in_rect(n, x, y) {
            turn_page(1);
        }
    }
}

unsafe fn play_clicked(h: Hwnd) {
    apply_scale_text();
    let mut text = get_text(EDIT);
    if text.is_empty() {
        if let Some(p) = browse(h) {
            text = p.to_string_lossy().to_string();
            set_text(EDIT, &text);
        }
    }
    if text.is_empty() {
        return;
    }
    save_ui();
    set_status("starting...");
    match play(Path::new(&text)) {
        Ok((msg, watch)) => {
            if let Some((pid, dir)) = watch {
                std::thread::spawn(move || watch_children(pid, &dir));
            }
            set_status(&msg);
        }
        Err(err) => set_status(&format!("error: {err}")),
    }
}

unsafe fn browse(owner: Hwnd) -> Option<PathBuf> {
    let mut buf = [0u16; 1024];
    let mut ofn: OpenFileNameW = zeroed();
    ofn.struct_size = size_of::<OpenFileNameW>() as u32;
    ofn.owner = owner;
    ofn.file = buf.as_mut_ptr();
    ofn.max_file = buf.len() as u32;
    let filter: Vec<u16> = "Games (*.exe)\0*.exe\0All files (*.*)\0*.*\0\0"
        .encode_utf16()
        .collect();
    let title = wide("Select the game .exe");

    let start: PathBuf = {
        let typed = PathBuf::from(get_text(EDIT));
        match typed.parent() {
            Some(d) if d.is_dir() => d.to_path_buf(),
            _ => exe_dir(),
        }
    };
    let start_dir = wide(&start.to_string_lossy());

    ofn.filter = filter.as_ptr();
    ofn.title = title.as_ptr();
    ofn.initial_dir = start_dir.as_ptr() as *mut u16;
    ofn.flags = OFN_FILEMUSTEXIST | OFN_PATHMUSTEXIST | OFN_EXPLORER | OFN_NOCHANGEDIR;

    SetForegroundWindow(owner);
    if GetOpenFileNameW(&mut ofn) == 0 {
        let err = CommDlgExtendedError();
        if err != 0 {
            set_status(&format!("browse failed (error 0x{err:X}) - type the path by hand"));
        }
        return None;
    }
    let n = buf.iter().position(|&c| c == 0).unwrap_or(buf.len());
    Some(PathBuf::from(String::from_utf16_lossy(&buf[..n])))
}

unsafe extern "system" fn wndproc(h: Hwnd, msg: u32, wp: usize, lp: isize) -> isize {
    match msg {
        WM_CREATE => {
            MAIN = h;
            let display = wide("Segoe UI Variable Display");
            let text = wide("Segoe UI");
            F_TITLE = CreateFontW(-28, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, display.as_ptr());
            F_HEAD = CreateFontW(-15, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, display.as_ptr());
            F_BODY = CreateFontW(-14, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0, text.as_ptr());
            F_SMALL = CreateFontW(-12, 0, 0, 0, 400, 0, 0, 0, 1, 0, 0, 5, 0, text.as_ptr());
            F_BIG = CreateFontW(-36, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, display.as_ptr());
            F_VALUE = CreateFontW(-24, 0, 0, 0, 600, 0, 0, 0, 1, 0, 0, 5, 0, display.as_ptr());
            FIELD_BRUSH = CreateSolidBrush(FIELD);

            let on: i32 = 1;
            DwmSetWindowAttribute(h, 20, &on as *const i32 as *const _, 4);
            DwmSetWindowAttribute(h, 19, &on as *const i32 as *const _, 4);
            let round = DWMWCP_ROUND;
            DwmSetWindowAttribute(h, DWMWA_WINDOW_CORNER_PREFERENCE, &round as *const u32 as *const _, 4);
            let caption = BG;
            let caption_text = TEXT;
            DwmSetWindowAttribute(h, DWMWA_CAPTION_COLOR, &caption as *const u32 as *const _, 4);
            DwmSetWindowAttribute(h, DWMWA_TEXT_COLOR, &caption_text as *const u32 as *const _, 4);

            make_controls(h);
            load_ui();
            SetTimer(h, TIMER_TICK, 400, ptr::null_mut());
            0
        }
        WM_TIMER => {
            let st = STATUS.lock().map(|s| s.clone()).unwrap_or_default();
            if st.starts_with("running") && !game_running() {
                set_status("game closed  •  ready");
            }
            0
        }
        WM_PAINT => {
            paint(h);
            0
        }
        WM_ERASEBKGND => 1,
        WM_DRAWITEM => {
            draw_button(lp as *const DrawItemStruct);
            1
        }
        WM_CTLCOLOREDIT => {
            let dc = wp as Hdc;
            unsafe {
                SetTextColor(dc, if lp as Hwnd == EDIT_SCALE { ACCENT } else { TEXT });
                SetBkColor(dc, FIELD);
            }
            FIELD_BRUSH as isize
        }
        WM_LBUTTONDOWN => {
            let x = (lp & 0xFFFF) as i16 as i32;
            let y = ((lp >> 16) & 0xFFFF) as i16 as i32;
            let r = SLIDER;
            if x >= r.left - 10 && x <= r.right + 10 && y >= r.top - 12 && y <= r.bottom + 20 {
                DRAGGING = true;
                SetCapture(h);
                set_scale_from_x(x);
            } else if let Some(i) = tile_at(x, y) {
                select_game(i);
            } else {
                header_click(h, x, y);
            }
            0
        }
        WM_LBUTTONDBLCLK => {
            let x = (lp & 0xFFFF) as i16 as i32;
            let y = ((lp >> 16) & 0xFFFF) as i16 as i32;
            if let Some(i) = tile_at(x, y) {
                select_game(i);
                play_clicked(h);
            }
            0
        }
        WM_RBUTTONDOWN => {
            let x = (lp & 0xFFFF) as i16 as i32;
            let y = ((lp >> 16) & 0xFFFF) as i16 as i32;
            if let Some(i) = tile_at(x, y) {
                remove_game(i);
            }
            0
        }
        WM_MOUSEMOVE => {
            if DRAGGING {
                let x = (lp & 0xFFFF) as i16 as i32;
                set_scale_from_x(x);
            }
            0
        }
        WM_LBUTTONUP => {
            if DRAGGING {
                DRAGGING = false;
                ReleaseCapture();
                save_ui();
            }
            0
        }
        WM_COMMAND => match (wp & 0xFFFF) as i32 {
            ID_BROWSE => {
                if let Some(p) = browse(h) {
                    set_text(EDIT, &p.to_string_lossy());
                    set_status("");
                    save_ui();
                }
                0
            }
            ID_SCALE => {
                let note = ((wp >> 16) & 0xFFFF) as u32;
                if note == EN_CHANGE && !SYNCING {
                    apply_scale_text();
                } else if note == EN_KILLFOCUS {
                    sync_scale_text();
                    save_ui();
                }
                0
            }
            ID_PLAY => {
                play_clicked(h);
                0
            }
            _ => 0,
        },
        WM_CLOSE => {
            save_ui();
            DefWindowProcW(h, msg, wp, lp)
        }
        WM_DESTROY => {
            save_ui();
            osd_clear();
            PostQuitMessage(0);
            0
        }
        _ => DefWindowProcW(h, msg, wp, lp),
    }
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();

    if let Some(pos) = args.iter().position(|a| a == "--patch") {
        if let Some(pid) = args.get(pos + 1).and_then(|p| p.parse::<u32>().ok()) {
            patch_loop(pid);
        }
        return;
    }

    let game: Option<String> = args
        .iter()
        .find(|a| !a.starts_with('-'))
        .map(|s| s.trim_matches('"').to_string());
    let cli = game.is_some() && args.iter().any(|a| a == "--play");

    if cli {
        USE_INI.store(true, Ordering::Relaxed);
        let g = game.unwrap_or_default();
        match play(Path::new(&g)) {
            Ok((_, watch)) => {
                if let Some((pid, dir)) = watch {
                    watch_children(pid, &dir);
                }
            }
            Err(err) => unsafe {
                MessageBoxW(
                    ptr::null_mut(),
                    wide(&err).as_ptr(),
                    wide("MFG").as_ptr(),
                    MB_ICONERROR,
                );
            },
        }
        return;
    }

    ensure_minimal_ini();
    set_scale(read_scale().clamp(1.0, 10.0));

    unsafe {
        let cursor = LoadCursorW(ptr::null_mut(), 32512 as *const u16);
        let icon = make_icon();
        let wc = WndClassW {
            style: CS_DBLCLKS,
            lpfn_wnd_proc: Some(wndproc),
            cb_cls_extra: 0,
            cb_wnd_extra: 0,
            h_instance: GetModuleHandleW(ptr::null()),
            h_icon: icon,
            h_cursor: cursor,
            hbr_background: GetSysColorBrush(15),
            menu_name: ptr::null(),
            class_name: CLASS_NAME.as_ptr(),
        };
        let atom = RegisterClassW(&wc);
        if atom == 0 {
            MessageBoxW(
                ptr::null_mut(),
                wide("could not register the window class").as_ptr(),
                wide("fps launcher").as_ptr(),
                MB_ICONERROR,
            );
            return;
        }

        let mut rc = Rect { left: 0, top: 0, right: 680, bottom: 620 };
        AdjustWindowRect(&mut rc, WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX, 0);
        let ww = rc.right - rc.left;
        let wh = rc.bottom - rc.top;
        let wx = (GetSystemMetrics(0) - ww) / 2;
        let wy = (GetSystemMetrics(1) - wh) / 2;
        let h = CreateWindowExW(
            0,
            CLASS_NAME.as_ptr(),
            wide("MFG").as_ptr(),
            WS_CAPTION | WS_SYSMENU | WS_MINIMIZEBOX,
            wx,
            wy,
            ww,
            wh,
            ptr::null_mut(),
            ptr::null_mut(),
            GetModuleHandleW(ptr::null()),
            ptr::null_mut(),
        );
        if h.is_null() {
            return;
        }
        if !icon.is_null() {
            SendMessageW(h, WM_SETICON, 1, icon as isize);
            SendMessageW(h, WM_SETICON, 0, icon as isize);
        }
        ShowWindow(h, SW_SHOW);
        UpdateWindow(h);

        if let Some(g) = game {
            set_text(EDIT, &g);
        }

        let mut msg: Msg = zeroed();
        while GetMessageW(&mut msg, ptr::null_mut(), 0, 0) > 0 {
            TranslateMessage(&msg);
            DispatchMessageW(&msg);
        }
    }
}
