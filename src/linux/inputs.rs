use ::*;
use KeybdKey::*;
use MouseButton::*;

impl From<KeybdKey> for u64 {
    fn from(key: KeybdKey) -> u64 {
        match key {
            BackspaceKey => 0xFF08,
            TabKey => 0xFF09,
            EnterKey => 0xFF8D,
            EscapeKey => 0xFF1B,
            SpaceKey => 0x020,
            HomeKey => 0xFF50,
            LeftKey => 0xFF51,
            UpKey => 0xFF52,
            RightKey => 0xFF53,
            DownKey => 0xFF54,
            InsertKey => 0xFF63,
            DeleteKey => 0xFF9F,
            Numrow0Key => 0x030,
            Numrow1Key => 0x031,
            Numrow2Key => 0x032,
            Numrow3Key => 0x033,
            Numrow4Key => 0x034,
            Numrow5Key => 0x035,
            Numrow6Key => 0x036,
            Numrow7Key => 0x037,
            Numrow8Key => 0x038,
            Numrow9Key => 0x039,
            AKey => 0x041,
            BKey => 0x042,
            CKey => 0x043,
            DKey => 0x044,
            EKey => 0x045,
            FKey => 0x046,
            GKey => 0x047,
            HKey => 0x048,
            IKey => 0x049,
            JKey => 0x04A,
            KKey => 0x04B,
            LKey => 0x04C,
            MKey => 0x04D,
            NKey => 0x04E,
            OKey => 0x04F,
            PKey => 0x050,
            QKey => 0x051,
            RKey => 0x052,
            SKey => 0x053,
            TKey => 0x054,
            UKey => 0x055,
            VKey => 0x056,
            WKey => 0x057,
            XKey => 0x058,
            YKey => 0x059,
            ZKey => 0x05A,
            Numpad0Key => 0xFFB0,
            Numpad1Key => 0xFFB1,
            Numpad2Key => 0xFFB2,
            Numpad3Key => 0xFFB3,
            Numpad4Key => 0xFFB4,
            Numpad5Key => 0xFFB5,
            Numpad6Key => 0xFFB6,
            Numpad7Key => 0xFFB7,
            Numpad8Key => 0xFFB8,
            Numpad9Key => 0xFFB9,
            F1Key => 0xFFBE,
            F2Key => 0xFFBF,
            F3Key => 0xFFC0,
            F4Key => 0xFFC1,
            F5Key => 0xFFC2,
            F6Key => 0xFFC3,
            F7Key => 0xFFC4,
            F8Key => 0xFFC5,
            F9Key => 0xFFC6,
            F10Key => 0xFFC7,
            F11Key => 0xFFC8,
            F12Key => 0xFFC9,
            NumLockKey => 0xFF7F,
            ScrollLockKey => 0xFF14,
            CapsLockKey => 0xFFE5,
            LShiftKey => 0xFFE1,
            RShiftKey => 0xFFE2,
            LControlKey => 0xFFE3,
            RControlKey => 0xFFE4,
            OtherKey(keycode) => keycode,
        }
    }
}

impl From<u32> for MouseButton {
    fn from(keycode: u32) -> MouseButton {
        match keycode {
            1 => LeftButton,
            2 => MiddleButton,
            3 => RightButton,
            4 => X1Button,
            5 => X2Button,
            _ => OtherButton(keycode),
        }
    }
}

impl From<MouseButton> for u32 {
    fn from(button: MouseButton) -> u32 {
        match button {
            LeftButton => 1,
            MiddleButton => 2,
            RightButton => 3,
            X1Button => 4,
            X2Button => 5,
            OtherButton(keycode) => keycode,
        }
    }
}
