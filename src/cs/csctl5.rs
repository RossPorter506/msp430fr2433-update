#[doc = "Register `CSCTL5` reader"]
pub struct R(crate::R<CSCTL5_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL5_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL5_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL5_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL5` writer"]
pub struct W(crate::W<CSCTL5_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL5_SPEC>;
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
impl From<crate::W<CSCTL5_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL5_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DIVM` reader - MCLK Divider Bit: 0"]
pub type DIVM_R = crate::FieldReader<u8, DIVM_A>;
#[doc = "MCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVM_A {
    #[doc = "0: MCLK Source Divider 0"]
    DIVM_0 = 0,
    #[doc = "1: MCLK Source Divider 1"]
    DIVM_1 = 1,
    #[doc = "2: MCLK Source Divider 2"]
    DIVM_2 = 2,
    #[doc = "3: MCLK Source Divider 3"]
    DIVM_3 = 3,
    #[doc = "4: MCLK Source Divider 4"]
    DIVM_4 = 4,
    #[doc = "5: MCLK Source Divider 5"]
    DIVM_5 = 5,
    #[doc = "6: MCLK Source Divider 6"]
    DIVM_6 = 6,
    #[doc = "7: MCLK Source Divider 7"]
    DIVM_7 = 7,
}
impl From<DIVM_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVM_A) -> Self {
        variant as _
    }
}
impl DIVM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVM_A {
        match self.bits {
            0 => DIVM_A::DIVM_0,
            1 => DIVM_A::DIVM_1,
            2 => DIVM_A::DIVM_2,
            3 => DIVM_A::DIVM_3,
            4 => DIVM_A::DIVM_4,
            5 => DIVM_A::DIVM_5,
            6 => DIVM_A::DIVM_6,
            7 => DIVM_A::DIVM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVM_0`"]
    #[inline(always)]
    pub fn is_divm_0(&self) -> bool {
        *self == DIVM_A::DIVM_0
    }
    #[doc = "Checks if the value of the field is `DIVM_1`"]
    #[inline(always)]
    pub fn is_divm_1(&self) -> bool {
        *self == DIVM_A::DIVM_1
    }
    #[doc = "Checks if the value of the field is `DIVM_2`"]
    #[inline(always)]
    pub fn is_divm_2(&self) -> bool {
        *self == DIVM_A::DIVM_2
    }
    #[doc = "Checks if the value of the field is `DIVM_3`"]
    #[inline(always)]
    pub fn is_divm_3(&self) -> bool {
        *self == DIVM_A::DIVM_3
    }
    #[doc = "Checks if the value of the field is `DIVM_4`"]
    #[inline(always)]
    pub fn is_divm_4(&self) -> bool {
        *self == DIVM_A::DIVM_4
    }
    #[doc = "Checks if the value of the field is `DIVM_5`"]
    #[inline(always)]
    pub fn is_divm_5(&self) -> bool {
        *self == DIVM_A::DIVM_5
    }
    #[doc = "Checks if the value of the field is `DIVM_6`"]
    #[inline(always)]
    pub fn is_divm_6(&self) -> bool {
        *self == DIVM_A::DIVM_6
    }
    #[doc = "Checks if the value of the field is `DIVM_7`"]
    #[inline(always)]
    pub fn is_divm_7(&self) -> bool {
        *self == DIVM_A::DIVM_7
    }
}
#[doc = "Field `DIVM` writer - MCLK Divider Bit: 0"]
pub type DIVM_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CSCTL5_SPEC, u8, DIVM_A, 3, O>;
impl<'a, const O: u8> DIVM_W<'a, O> {
    #[doc = "MCLK Source Divider 0"]
    #[inline(always)]
    pub fn divm_0(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_0)
    }
    #[doc = "MCLK Source Divider 1"]
    #[inline(always)]
    pub fn divm_1(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_1)
    }
    #[doc = "MCLK Source Divider 2"]
    #[inline(always)]
    pub fn divm_2(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_2)
    }
    #[doc = "MCLK Source Divider 3"]
    #[inline(always)]
    pub fn divm_3(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_3)
    }
    #[doc = "MCLK Source Divider 4"]
    #[inline(always)]
    pub fn divm_4(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_4)
    }
    #[doc = "MCLK Source Divider 5"]
    #[inline(always)]
    pub fn divm_5(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_5)
    }
    #[doc = "MCLK Source Divider 6"]
    #[inline(always)]
    pub fn divm_6(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_6)
    }
    #[doc = "MCLK Source Divider 7"]
    #[inline(always)]
    pub fn divm_7(self) -> &'a mut W {
        self.variant(DIVM_A::DIVM_7)
    }
}
#[doc = "Field `DIVS` reader - SMCLK Divider Bit: 0"]
pub type DIVS_R = crate::FieldReader<u8, DIVS_A>;
#[doc = "SMCLK Divider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DIVS_A {
    #[doc = "0: SMCLK Source Divider 0"]
    DIVS_0 = 0,
    #[doc = "1: SMCLK Source Divider 1"]
    DIVS_1 = 1,
    #[doc = "2: SMCLK Source Divider 2"]
    DIVS_2 = 2,
    #[doc = "3: SMCLK Source Divider 3"]
    DIVS_3 = 3,
}
impl From<DIVS_A> for u8 {
    #[inline(always)]
    fn from(variant: DIVS_A) -> Self {
        variant as _
    }
}
impl DIVS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DIVS_A {
        match self.bits {
            0 => DIVS_A::DIVS_0,
            1 => DIVS_A::DIVS_1,
            2 => DIVS_A::DIVS_2,
            3 => DIVS_A::DIVS_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DIVS_0`"]
    #[inline(always)]
    pub fn is_divs_0(&self) -> bool {
        *self == DIVS_A::DIVS_0
    }
    #[doc = "Checks if the value of the field is `DIVS_1`"]
    #[inline(always)]
    pub fn is_divs_1(&self) -> bool {
        *self == DIVS_A::DIVS_1
    }
    #[doc = "Checks if the value of the field is `DIVS_2`"]
    #[inline(always)]
    pub fn is_divs_2(&self) -> bool {
        *self == DIVS_A::DIVS_2
    }
    #[doc = "Checks if the value of the field is `DIVS_3`"]
    #[inline(always)]
    pub fn is_divs_3(&self) -> bool {
        *self == DIVS_A::DIVS_3
    }
}
#[doc = "Field `DIVS` writer - SMCLK Divider Bit: 0"]
pub type DIVS_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u16, CSCTL5_SPEC, u8, DIVS_A, 2, O>;
impl<'a, const O: u8> DIVS_W<'a, O> {
    #[doc = "SMCLK Source Divider 0"]
    #[inline(always)]
    pub fn divs_0(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_0)
    }
    #[doc = "SMCLK Source Divider 1"]
    #[inline(always)]
    pub fn divs_1(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_1)
    }
    #[doc = "SMCLK Source Divider 2"]
    #[inline(always)]
    pub fn divs_2(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_2)
    }
    #[doc = "SMCLK Source Divider 3"]
    #[inline(always)]
    pub fn divs_3(self) -> &'a mut W {
        self.variant(DIVS_A::DIVS_3)
    }
}
#[doc = "Field `SMCLKOFF` reader - SMCLK off"]
pub type SMCLKOFF_R = crate::BitReader<bool>;
#[doc = "Field `SMCLKOFF` writer - SMCLK off"]
pub type SMCLKOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL5_SPEC, bool, O>;
#[doc = "Field `VLOAUTOOFF` reader - VLO automatic off enable"]
pub type VLOAUTOOFF_R = crate::BitReader<bool>;
#[doc = "Field `VLOAUTOOFF` writer - VLO automatic off enable"]
pub type VLOAUTOOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL5_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&self) -> DIVM_R {
        DIVM_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&self) -> DIVS_R {
        DIVS_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&self) -> SMCLKOFF_R {
        SMCLKOFF_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline(always)]
    pub fn vloautooff(&self) -> VLOAUTOOFF_R {
        VLOAUTOOFF_R::new(((self.bits >> 12) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divm(&mut self) -> DIVM_W<0> {
        DIVM_W::new(self)
    }
    #[doc = "Bits 4:5 - SMCLK Divider Bit: 0"]
    #[inline(always)]
    pub fn divs(&mut self) -> DIVS_W<4> {
        DIVS_W::new(self)
    }
    #[doc = "Bit 8 - SMCLK off"]
    #[inline(always)]
    pub fn smclkoff(&mut self) -> SMCLKOFF_W<8> {
        SMCLKOFF_W::new(self)
    }
    #[doc = "Bit 12 - VLO automatic off enable"]
    #[inline(always)]
    pub fn vloautooff(&mut self) -> VLOAUTOOFF_W<12> {
        VLOAUTOOFF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 5\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl5](index.html) module"]
pub struct CSCTL5_SPEC;
impl crate::RegisterSpec for CSCTL5_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl5::R](R) reader structure"]
impl crate::Readable for CSCTL5_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl5::W](W) writer structure"]
impl crate::Writable for CSCTL5_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL5 to value 0"]
impl crate::Resettable for CSCTL5_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
