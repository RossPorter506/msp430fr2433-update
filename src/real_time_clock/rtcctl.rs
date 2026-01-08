#[doc = "Register `RTCCTL` reader"]
pub struct R(crate::R<RTCCTL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RTCCTL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RTCCTL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RTCCTL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RTCCTL` writer"]
pub struct W(crate::W<RTCCTL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RTCCTL_SPEC>;
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
impl From<crate::W<RTCCTL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RTCCTL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTCIF` reader - Low-Power-Counter Interrupt Flag"]
pub type RTCIF_R = crate::BitReader<bool>;
#[doc = "Field `RTCIF` writer - Low-Power-Counter Interrupt Flag"]
pub type RTCIF_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL_SPEC, bool, O>;
#[doc = "Field `RTCIE` reader - Low-Power-Counter Interrupt Enable"]
pub type RTCIE_R = crate::BitReader<bool>;
#[doc = "Field `RTCIE` writer - Low-Power-Counter Interrupt Enable"]
pub type RTCIE_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL_SPEC, bool, O>;
#[doc = "Field `RTCSR` reader - Low-Power-Counter Software Reset"]
pub type RTCSR_R = crate::BitReader<bool>;
#[doc = "Field `RTCSR` writer - Low-Power-Counter Software Reset"]
pub type RTCSR_W<'a, const O: u8> = crate::BitWriter<'a, u16, RTCCTL_SPEC, bool, O>;
#[doc = "Field `RTCPS` reader - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RTCPS_R = crate::FieldReader<u8, RTCPS_A>;
#[doc = "Low-Power-Counter Clock Pre-divider Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCPS_A {
    #[doc = "0: Low-Power-Counter Clock Pre-divider Select: 0"]
    RTCPS_0 = 0,
    #[doc = "1: Low-Power-Counter Clock Pre-divider Select: 1"]
    RTCPS_1 = 1,
    #[doc = "2: Low-Power-Counter Clock Pre-divider Select: 2"]
    RTCPS_2 = 2,
    #[doc = "3: Low-Power-Counter Clock Pre-divider Select: 3"]
    RTCPS_3 = 3,
    #[doc = "4: Low-Power-Counter Clock Pre-divider Select: 4"]
    RTCPS_4 = 4,
    #[doc = "5: Low-Power-Counter Clock Pre-divider Select: 5"]
    RTCPS_5 = 5,
    #[doc = "6: Low-Power-Counter Clock Pre-divider Select: 6"]
    RTCPS_6 = 6,
    #[doc = "7: Low-Power-Counter Clock Pre-divider Select: 7"]
    RTCPS_7 = 7,
}
impl From<RTCPS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCPS_A) -> Self {
        variant as _
    }
}
impl RTCPS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCPS_A {
        match self.bits {
            0 => RTCPS_A::RTCPS_0,
            1 => RTCPS_A::RTCPS_1,
            2 => RTCPS_A::RTCPS_2,
            3 => RTCPS_A::RTCPS_3,
            4 => RTCPS_A::RTCPS_4,
            5 => RTCPS_A::RTCPS_5,
            6 => RTCPS_A::RTCPS_6,
            7 => RTCPS_A::RTCPS_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCPS_0`"]
    #[inline(always)]
    pub fn is_rtcps_0(&self) -> bool {
        *self == RTCPS_A::RTCPS_0
    }
    #[doc = "Checks if the value of the field is `RTCPS_1`"]
    #[inline(always)]
    pub fn is_rtcps_1(&self) -> bool {
        *self == RTCPS_A::RTCPS_1
    }
    #[doc = "Checks if the value of the field is `RTCPS_2`"]
    #[inline(always)]
    pub fn is_rtcps_2(&self) -> bool {
        *self == RTCPS_A::RTCPS_2
    }
    #[doc = "Checks if the value of the field is `RTCPS_3`"]
    #[inline(always)]
    pub fn is_rtcps_3(&self) -> bool {
        *self == RTCPS_A::RTCPS_3
    }
    #[doc = "Checks if the value of the field is `RTCPS_4`"]
    #[inline(always)]
    pub fn is_rtcps_4(&self) -> bool {
        *self == RTCPS_A::RTCPS_4
    }
    #[doc = "Checks if the value of the field is `RTCPS_5`"]
    #[inline(always)]
    pub fn is_rtcps_5(&self) -> bool {
        *self == RTCPS_A::RTCPS_5
    }
    #[doc = "Checks if the value of the field is `RTCPS_6`"]
    #[inline(always)]
    pub fn is_rtcps_6(&self) -> bool {
        *self == RTCPS_A::RTCPS_6
    }
    #[doc = "Checks if the value of the field is `RTCPS_7`"]
    #[inline(always)]
    pub fn is_rtcps_7(&self) -> bool {
        *self == RTCPS_A::RTCPS_7
    }
}
#[doc = "Field `RTCPS` writer - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
pub type RTCPS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, RTCCTL_SPEC, u8, RTCPS_A, 3, O>;
impl<'a, const O: u8> RTCPS_W<'a, O> {
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 0"]
    #[inline(always)]
    pub fn rtcps_0(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_0)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 1"]
    #[inline(always)]
    pub fn rtcps_1(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_1)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 2"]
    #[inline(always)]
    pub fn rtcps_2(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_2)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 3"]
    #[inline(always)]
    pub fn rtcps_3(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_3)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 4"]
    #[inline(always)]
    pub fn rtcps_4(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_4)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 5"]
    #[inline(always)]
    pub fn rtcps_5(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_5)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 6"]
    #[inline(always)]
    pub fn rtcps_6(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_6)
    }
    #[doc = "Low-Power-Counter Clock Pre-divider Select: 7"]
    #[inline(always)]
    pub fn rtcps_7(self) -> &'a mut W {
        self.variant(RTCPS_A::RTCPS_7)
    }
}
#[doc = "Field `RTCSS` reader - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RTCSS_R = crate::FieldReader<u8, RTCSS_A>;
#[doc = "Low-Power-Counter Clock Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum RTCSS_A {
    #[doc = "0: Low-Power-Counter Clock Source Select: 0"]
    RTCSS_0 = 0,
    #[doc = "1: Low-Power-Counter Clock Source Select: 1"]
    RTCSS_1 = 1,
    #[doc = "2: Low-Power-Counter Clock Source Select: 2"]
    RTCSS_2 = 2,
    #[doc = "3: Low-Power-Counter Clock Source Select: 3"]
    RTCSS_3 = 3,
}
impl From<RTCSS_A> for u8 {
    #[inline(always)]
    fn from(variant: RTCSS_A) -> Self {
        variant as _
    }
}
impl RTCSS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> RTCSS_A {
        match self.bits {
            0 => RTCSS_A::RTCSS_0,
            1 => RTCSS_A::RTCSS_1,
            2 => RTCSS_A::RTCSS_2,
            3 => RTCSS_A::RTCSS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `RTCSS_0`"]
    #[inline(always)]
    pub fn is_rtcss_0(&self) -> bool {
        *self == RTCSS_A::RTCSS_0
    }
    #[doc = "Checks if the value of the field is `RTCSS_1`"]
    #[inline(always)]
    pub fn is_rtcss_1(&self) -> bool {
        *self == RTCSS_A::RTCSS_1
    }
    #[doc = "Checks if the value of the field is `RTCSS_2`"]
    #[inline(always)]
    pub fn is_rtcss_2(&self) -> bool {
        *self == RTCSS_A::RTCSS_2
    }
    #[doc = "Checks if the value of the field is `RTCSS_3`"]
    #[inline(always)]
    pub fn is_rtcss_3(&self) -> bool {
        *self == RTCSS_A::RTCSS_3
    }
}
#[doc = "Field `RTCSS` writer - Low-Power-Counter Clock Source Select Bit: 0"]
pub type RTCSS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, RTCCTL_SPEC, u8, RTCSS_A, 2, O>;
impl<'a, const O: u8> RTCSS_W<'a, O> {
    #[doc = "Low-Power-Counter Clock Source Select: 0"]
    #[inline(always)]
    pub fn rtcss_0(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_0)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 1"]
    #[inline(always)]
    pub fn rtcss_1(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_1)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 2"]
    #[inline(always)]
    pub fn rtcss_2(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_2)
    }
    #[doc = "Low-Power-Counter Clock Source Select: 3"]
    #[inline(always)]
    pub fn rtcss_3(self) -> &'a mut W {
        self.variant(RTCSS_A::RTCSS_3)
    }
}
impl R {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&self) -> RTCIF_R {
        RTCIF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&self) -> RTCIE_R {
        RTCIE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&self) -> RTCSR_R {
        RTCSR_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&self) -> RTCPS_R {
        RTCPS_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&self) -> RTCSS_R {
        RTCSS_R::new(((self.bits >> 12) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - Low-Power-Counter Interrupt Flag"]
    #[inline(always)]
    pub fn rtcif(&mut self) -> RTCIF_W<0> {
        RTCIF_W::new(self)
    }
    #[doc = "Bit 1 - Low-Power-Counter Interrupt Enable"]
    #[inline(always)]
    pub fn rtcie(&mut self) -> RTCIE_W<1> {
        RTCIE_W::new(self)
    }
    #[doc = "Bit 6 - Low-Power-Counter Software Reset"]
    #[inline(always)]
    pub fn rtcsr(&mut self) -> RTCSR_W<6> {
        RTCSR_W::new(self)
    }
    #[doc = "Bits 8:10 - Low-Power-Counter Clock Pre-divider Select Bit: 0"]
    #[inline(always)]
    pub fn rtcps(&mut self) -> RTCPS_W<8> {
        RTCPS_W::new(self)
    }
    #[doc = "Bits 12:13 - Low-Power-Counter Clock Source Select Bit: 0"]
    #[inline(always)]
    pub fn rtcss(&mut self) -> RTCSS_W<12> {
        RTCSS_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC control Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rtcctl](index.html) module"]
pub struct RTCCTL_SPEC;
impl crate::RegisterSpec for RTCCTL_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [rtcctl::R](R) reader structure"]
impl crate::Readable for RTCCTL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rtcctl::W](W) writer structure"]
impl crate::Writable for RTCCTL_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets RTCCTL to value 0"]
impl crate::Resettable for RTCCTL_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
