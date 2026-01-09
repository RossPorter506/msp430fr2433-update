#[doc = "Register `WDTCTL` reader"]
pub struct R(crate::R<WDTCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCTL` writer"]
pub struct W(crate::W<WDTCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<WDTCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDTIS` reader - WDT - Timer Interval Select 0"]
pub type WDTIS_R = crate::FieldReader<u8, WDTIS_A>;
#[doc = "WDT - Timer Interval Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTIS_A {
    #[doc = "0: WDT - Timer Interval Select: Watchdog clock source /2048M"]
    _2048M = 0,
    #[doc = "1: WDT - Timer Interval Select: Watchdog clock source /128M"]
    _128M = 1,
    #[doc = "2: WDT - Timer Interval Select: Watchdog clock source /8192K"]
    _8192K = 2,
    #[doc = "3: WDT - Timer Interval Select: Watchdog clock source /512K"]
    _512K = 3,
    #[doc = "4: WDT - Timer Interval Select: Watchdog clock source /32K"]
    _32K = 4,
    #[doc = "5: WDT - Timer Interval Select: Watchdog clock source /8192K"]
    _8192 = 5,
    #[doc = "6: WDT - Timer Interval Select: Watchdog clock source /512"]
    _512 = 6,
    #[doc = "7: WDT - Timer Interval Select: Watchdog clock source /64"]
    _64 = 7,
}
impl From<WDTIS_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTIS_A) -> Self {
        variant as _
    }
}
impl WDTIS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTIS_A {
        match self.bits {
            0 => WDTIS_A::_2048M,
            1 => WDTIS_A::_128M,
            2 => WDTIS_A::_8192K,
            3 => WDTIS_A::_512K,
            4 => WDTIS_A::_32K,
            5 => WDTIS_A::_8192,
            6 => WDTIS_A::_512,
            7 => WDTIS_A::_64,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `_2048M`"]
    #[inline(always)]
    pub fn is_2048m(&self) -> bool {
        *self == WDTIS_A::_2048M
    }
    #[doc = "Checks if the value of the field is `_128M`"]
    #[inline(always)]
    pub fn is_128m(&self) -> bool {
        *self == WDTIS_A::_128M
    }
    #[doc = "Checks if the value of the field is `_8192K`"]
    #[inline(always)]
    pub fn is_8192k(&self) -> bool {
        *self == WDTIS_A::_8192K
    }
    #[doc = "Checks if the value of the field is `_512K`"]
    #[inline(always)]
    pub fn is_512k(&self) -> bool {
        *self == WDTIS_A::_512K
    }
    #[doc = "Checks if the value of the field is `_32K`"]
    #[inline(always)]
    pub fn is_32k(&self) -> bool {
        *self == WDTIS_A::_32K
    }
    #[doc = "Checks if the value of the field is `_8192`"]
    #[inline(always)]
    pub fn is_8192(&self) -> bool {
        *self == WDTIS_A::_8192
    }
    #[doc = "Checks if the value of the field is `_512`"]
    #[inline(always)]
    pub fn is_512(&self) -> bool {
        *self == WDTIS_A::_512
    }
    #[doc = "Checks if the value of the field is `_64`"]
    #[inline(always)]
    pub fn is_64(&self) -> bool {
        *self == WDTIS_A::_64
    }
}
#[doc = "Field `WDTIS` writer - WDT - Timer Interval Select 0"]
pub type WDTIS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, WDTCTL_SPEC, u8, WDTIS_A, 3, O>;
impl<'a, const O: u8> WDTIS_W<'a, O> {
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /2048M"]
    #[inline(always)]
    pub fn _2048m(self) -> &'a mut W {
        self.variant(WDTIS_A::_2048M)
    }
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /128M"]
    #[inline(always)]
    pub fn _128m(self) -> &'a mut W {
        self.variant(WDTIS_A::_128M)
    }
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /8192K"]
    #[inline(always)]
    pub fn _8192k(self) -> &'a mut W {
        self.variant(WDTIS_A::_8192K)
    }
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /512K"]
    #[inline(always)]
    pub fn _512k(self) -> &'a mut W {
        self.variant(WDTIS_A::_512K)
    }
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /32K"]
    #[inline(always)]
    pub fn _32k(self) -> &'a mut W {
        self.variant(WDTIS_A::_32K)
    }
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /8192K"]
    #[inline(always)]
    pub fn _8192(self) -> &'a mut W {
        self.variant(WDTIS_A::_8192)
    }
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /512"]
    #[inline(always)]
    pub fn _512(self) -> &'a mut W {
        self.variant(WDTIS_A::_512)
    }
    #[doc = "WDT - Timer Interval Select: Watchdog clock source /64"]
    #[inline(always)]
    pub fn _64(self) -> &'a mut W {
        self.variant(WDTIS_A::_64)
    }
}
#[doc = "Field `WDTCNTCL` reader - WDT - Timer Clear"]
pub type WDTCNTCL_R = crate::BitReader<bool>;
#[doc = "Field `WDTCNTCL` writer - WDT - Timer Clear"]
pub type WDTCNTCL_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTTMSEL` reader - WDT - Timer Mode Select"]
pub type WDTTMSEL_R = crate::BitReader<bool>;
#[doc = "Field `WDTTMSEL` writer - WDT - Timer Mode Select"]
pub type WDTTMSEL_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, bool, O>;
#[doc = "Field `WDTSSEL` reader - WDT - Timer Clock Source Select 0"]
pub type WDTSSEL_R = crate::FieldReader<u8, WDTSSEL_A>;
#[doc = "WDT - Timer Clock Source Select 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTSSEL_A {
    #[doc = "0: WDT - Timer Clock Source Select: SMCLK"]
    SMCLK = 0,
    #[doc = "1: WDT - Timer Clock Source Select: ACLK"]
    ACLK = 1,
    #[doc = "2: WDT - Timer Clock Source Select: VLOCLK"]
    VLOCLK = 2,
    #[doc = "3: WDT - Timer Clock Source Select: Reserved"]
    X_CLK = 3,
}
impl From<WDTSSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTSSEL_A) -> Self {
        variant as _
    }
}
impl WDTSSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTSSEL_A {
        match self.bits {
            0 => WDTSSEL_A::SMCLK,
            1 => WDTSSEL_A::ACLK,
            2 => WDTSSEL_A::VLOCLK,
            3 => WDTSSEL_A::X_CLK,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `SMCLK`"]
    #[inline(always)]
    pub fn is_smclk(&self) -> bool {
        *self == WDTSSEL_A::SMCLK
    }
    #[doc = "Checks if the value of the field is `ACLK`"]
    #[inline(always)]
    pub fn is_aclk(&self) -> bool {
        *self == WDTSSEL_A::ACLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == WDTSSEL_A::VLOCLK
    }
    #[doc = "Checks if the value of the field is `X_CLK`"]
    #[inline(always)]
    pub fn is_x_clk(&self) -> bool {
        *self == WDTSSEL_A::X_CLK
    }
}
#[doc = "Field `WDTSSEL` writer - WDT - Timer Clock Source Select 0"]
pub type WDTSSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, WDTCTL_SPEC, u8, WDTSSEL_A, 2, O>;
impl<'a, const O: u8> WDTSSEL_W<'a, O> {
    #[doc = "WDT - Timer Clock Source Select: SMCLK"]
    #[inline(always)]
    pub fn smclk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::SMCLK)
    }
    #[doc = "WDT - Timer Clock Source Select: ACLK"]
    #[inline(always)]
    pub fn aclk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::ACLK)
    }
    #[doc = "WDT - Timer Clock Source Select: VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::VLOCLK)
    }
    #[doc = "WDT - Timer Clock Source Select: Reserved"]
    #[inline(always)]
    pub fn x_clk(self) -> &'a mut W {
        self.variant(WDTSSEL_A::X_CLK)
    }
}
#[doc = "Field `WDTHOLD` reader - WDT - Timer hold"]
pub type WDTHOLD_R = crate::BitReader<WDTHOLD_A>;
#[doc = "WDT - Timer hold\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WDTHOLD_A {
    #[doc = "0: Watchdog timer is not stopped"]
    UNHOLD = 0,
    #[doc = "1: Watchdog timer is stopped"]
    HOLD = 1,
}
impl From<WDTHOLD_A> for bool {
    #[inline(always)]
    fn from(variant: WDTHOLD_A) -> Self {
        variant as u8 != 0
    }
}
impl WDTHOLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> WDTHOLD_A {
        match self.bits {
            false => WDTHOLD_A::UNHOLD,
            true => WDTHOLD_A::HOLD,
        }
    }
    #[doc = "Checks if the value of the field is `UNHOLD`"]
    #[inline(always)]
    pub fn is_unhold(&self) -> bool {
        *self == WDTHOLD_A::UNHOLD
    }
    #[doc = "Checks if the value of the field is `HOLD`"]
    #[inline(always)]
    pub fn is_hold(&self) -> bool {
        *self == WDTHOLD_A::HOLD
    }
}
#[doc = "Field `WDTHOLD` writer - WDT - Timer hold"]
pub type WDTHOLD_W<'a, const O: u8> = crate::BitWriter<'a, u16, WDTCTL_SPEC, WDTHOLD_A, O>;
impl<'a, const O: u8> WDTHOLD_W<'a, O> {
    #[doc = "Watchdog timer is not stopped"]
    #[inline(always)]
    pub fn unhold(self) -> &'a mut W {
        self.variant(WDTHOLD_A::UNHOLD)
    }
    #[doc = "Watchdog timer is stopped"]
    #[inline(always)]
    pub fn hold(self) -> &'a mut W {
        self.variant(WDTHOLD_A::HOLD)
    }
}
#[doc = "Field `WDTPW` reader - Watchdog Timer Password"]
pub type WDTPW_R = crate::FieldReader<u8, WDTPWR_A>;
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWR_A {
    #[doc = "105: Value always read from the Watchdog Password register"]
    PASSWORD = 105,
}
impl From<WDTPWR_A> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWR_A) -> Self {
        variant as _
    }
}
impl WDTPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<WDTPWR_A> {
        match self.bits {
            105 => Some(WDTPWR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWORD`"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == WDTPWR_A::PASSWORD
    }
}
#[doc = "Watchdog Timer Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum WDTPWW_AW {
    #[doc = "90: Value which must be written to the Watchdog Password register"]
    PASSWORD = 90,
}
impl From<WDTPWW_AW> for u8 {
    #[inline(always)]
    fn from(variant: WDTPWW_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `WDTPW` writer - Watchdog Timer Password"]
pub type WDTPW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, WDTCTL_SPEC, u8, WDTPWW_AW, 8, O>;
impl<'a, const O: u8> WDTPW_W<'a, O> {
    #[doc = "Value which must be written to the Watchdog Password register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut W {
        self.variant(WDTPWW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&self) -> WDTIS_R {
        WDTIS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&self) -> WDTCNTCL_R {
        WDTCNTCL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&self) -> WDTTMSEL_R {
        WDTTMSEL_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&self) -> WDTSSEL_R {
        WDTSSEL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&self) -> WDTHOLD_R {
        WDTHOLD_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&self) -> WDTPW_R {
        WDTPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - WDT - Timer Interval Select 0"]
    #[inline(always)]
    pub fn wdtis(&mut self) -> WDTIS_W<0> {
        WDTIS_W::new(self)
    }
    #[doc = "Bit 3 - WDT - Timer Clear"]
    #[inline(always)]
    pub fn wdtcntcl(&mut self) -> WDTCNTCL_W<3> {
        WDTCNTCL_W::new(self)
    }
    #[doc = "Bit 4 - WDT - Timer Mode Select"]
    #[inline(always)]
    pub fn wdttmsel(&mut self) -> WDTTMSEL_W<4> {
        WDTTMSEL_W::new(self)
    }
    #[doc = "Bits 5:6 - WDT - Timer Clock Source Select 0"]
    #[inline(always)]
    pub fn wdtssel(&mut self) -> WDTSSEL_W<5> {
        WDTSSEL_W::new(self)
    }
    #[doc = "Bit 7 - WDT - Timer hold"]
    #[inline(always)]
    pub fn wdthold(&mut self) -> WDTHOLD_W<7> {
        WDTHOLD_W::new(self)
    }
    #[doc = "Bits 8:15 - Watchdog Timer Password"]
    #[inline(always)]
    pub fn wdtpw(&mut self) -> WDTPW_W<8> {
        WDTPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Watchdog Timer Control\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtctl](index.html) module"]
pub struct WDTCTL_SPEC;
impl crate::RegisterSpec for WDTCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [wdtctl::R](R) reader structure"]
impl crate::Readable for WDTCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtctl::W](W) writer structure"]
impl crate::Writable for WDTCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets WDTCTL to value 0"]
impl crate::Resettable for WDTCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
