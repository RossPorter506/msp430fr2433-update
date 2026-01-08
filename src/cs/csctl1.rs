#[doc = "Register `CSCTL1` reader"]
pub struct R(crate::R<CSCTL1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL1` writer"]
pub struct W(crate::W<CSCTL1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL1_SPEC>;
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
impl From<crate::W<CSCTL1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `DISMOD` reader - Disable Modulation"]
pub type DISMOD_R = crate::BitReader<bool>;
#[doc = "Field `DISMOD` writer - Disable Modulation"]
pub type DISMOD_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL1_SPEC, bool, O>;
#[doc = "Field `DCORSEL` reader - DCO frequency range select Bit: 0"]
pub type DCORSEL_R = crate::FieldReader<u8, DCORSEL_A>;
#[doc = "DCO frequency range select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCORSEL_A {
    #[doc = "0: DCO frequency range select: 0"]
    DCORSEL_0 = 0,
    #[doc = "1: DCO frequency range select: 1"]
    DCORSEL_1 = 1,
    #[doc = "2: DCO frequency range select: 2"]
    DCORSEL_2 = 2,
    #[doc = "3: DCO frequency range select: 3"]
    DCORSEL_3 = 3,
    #[doc = "4: DCO frequency range select: 4"]
    DCORSEL_4 = 4,
    #[doc = "5: DCO frequency range select: 5"]
    DCORSEL_5 = 5,
    #[doc = "6: DCO frequency range select: 6"]
    DCORSEL_6 = 6,
    #[doc = "7: DCO frequency range select: 7"]
    DCORSEL_7 = 7,
}
impl From<DCORSEL_A> for u8 {
    #[inline(always)]
    fn from(variant: DCORSEL_A) -> Self {
        variant as _
    }
}
impl DCORSEL_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCORSEL_A {
        match self.bits {
            0 => DCORSEL_A::DCORSEL_0,
            1 => DCORSEL_A::DCORSEL_1,
            2 => DCORSEL_A::DCORSEL_2,
            3 => DCORSEL_A::DCORSEL_3,
            4 => DCORSEL_A::DCORSEL_4,
            5 => DCORSEL_A::DCORSEL_5,
            6 => DCORSEL_A::DCORSEL_6,
            7 => DCORSEL_A::DCORSEL_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCORSEL_0`"]
    #[inline(always)]
    pub fn is_dcorsel_0(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_0
    }
    #[doc = "Checks if the value of the field is `DCORSEL_1`"]
    #[inline(always)]
    pub fn is_dcorsel_1(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_1
    }
    #[doc = "Checks if the value of the field is `DCORSEL_2`"]
    #[inline(always)]
    pub fn is_dcorsel_2(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_2
    }
    #[doc = "Checks if the value of the field is `DCORSEL_3`"]
    #[inline(always)]
    pub fn is_dcorsel_3(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_3
    }
    #[doc = "Checks if the value of the field is `DCORSEL_4`"]
    #[inline(always)]
    pub fn is_dcorsel_4(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_4
    }
    #[doc = "Checks if the value of the field is `DCORSEL_5`"]
    #[inline(always)]
    pub fn is_dcorsel_5(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_5
    }
    #[doc = "Checks if the value of the field is `DCORSEL_6`"]
    #[inline(always)]
    pub fn is_dcorsel_6(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_6
    }
    #[doc = "Checks if the value of the field is `DCORSEL_7`"]
    #[inline(always)]
    pub fn is_dcorsel_7(&self) -> bool {
        *self == DCORSEL_A::DCORSEL_7
    }
}
#[doc = "Field `DCORSEL` writer - DCO frequency range select Bit: 0"]
pub type DCORSEL_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL1_SPEC, u8, DCORSEL_A, 3, O>;
impl<'a, const O: u8> DCORSEL_W<'a, O> {
    #[doc = "DCO frequency range select: 0"]
    #[inline(always)]
    pub fn dcorsel_0(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_0)
    }
    #[doc = "DCO frequency range select: 1"]
    #[inline(always)]
    pub fn dcorsel_1(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_1)
    }
    #[doc = "DCO frequency range select: 2"]
    #[inline(always)]
    pub fn dcorsel_2(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_2)
    }
    #[doc = "DCO frequency range select: 3"]
    #[inline(always)]
    pub fn dcorsel_3(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_3)
    }
    #[doc = "DCO frequency range select: 4"]
    #[inline(always)]
    pub fn dcorsel_4(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_4)
    }
    #[doc = "DCO frequency range select: 5"]
    #[inline(always)]
    pub fn dcorsel_5(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_5)
    }
    #[doc = "DCO frequency range select: 6"]
    #[inline(always)]
    pub fn dcorsel_6(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_6)
    }
    #[doc = "DCO frequency range select: 7"]
    #[inline(always)]
    pub fn dcorsel_7(self) -> &'a mut W {
        self.variant(DCORSEL_A::DCORSEL_7)
    }
}
#[doc = "Field `DCOFTRIM` reader - DCO frequency trim. Bit: 0"]
pub type DCOFTRIM_R = crate::FieldReader<u8, DCOFTRIM_A>;
#[doc = "DCO frequency trim. Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum DCOFTRIM_A {
    #[doc = "0: DCO frequency trim: 0"]
    DCOFTRIM_0 = 0,
    #[doc = "1: DCO frequency trim: 1"]
    DCOFTRIM_1 = 1,
    #[doc = "2: DCO frequency trim: 2"]
    DCOFTRIM_2 = 2,
    #[doc = "3: DCO frequency trim: 3"]
    DCOFTRIM_3 = 3,
    #[doc = "4: DCO frequency trim: 4"]
    DCOFTRIM_4 = 4,
    #[doc = "5: DCO frequency trim: 5"]
    DCOFTRIM_5 = 5,
    #[doc = "6: DCO frequency trim: 6"]
    DCOFTRIM_6 = 6,
    #[doc = "7: DCO frequency trim: 7"]
    DCOFTRIM_7 = 7,
}
impl From<DCOFTRIM_A> for u8 {
    #[inline(always)]
    fn from(variant: DCOFTRIM_A) -> Self {
        variant as _
    }
}
impl DCOFTRIM_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> DCOFTRIM_A {
        match self.bits {
            0 => DCOFTRIM_A::DCOFTRIM_0,
            1 => DCOFTRIM_A::DCOFTRIM_1,
            2 => DCOFTRIM_A::DCOFTRIM_2,
            3 => DCOFTRIM_A::DCOFTRIM_3,
            4 => DCOFTRIM_A::DCOFTRIM_4,
            5 => DCOFTRIM_A::DCOFTRIM_5,
            6 => DCOFTRIM_A::DCOFTRIM_6,
            7 => DCOFTRIM_A::DCOFTRIM_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_0`"]
    #[inline(always)]
    pub fn is_dcoftrim_0(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_0
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_1`"]
    #[inline(always)]
    pub fn is_dcoftrim_1(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_1
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_2`"]
    #[inline(always)]
    pub fn is_dcoftrim_2(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_2
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_3`"]
    #[inline(always)]
    pub fn is_dcoftrim_3(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_3
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_4`"]
    #[inline(always)]
    pub fn is_dcoftrim_4(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_4
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_5`"]
    #[inline(always)]
    pub fn is_dcoftrim_5(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_5
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_6`"]
    #[inline(always)]
    pub fn is_dcoftrim_6(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_6
    }
    #[doc = "Checks if the value of the field is `DCOFTRIM_7`"]
    #[inline(always)]
    pub fn is_dcoftrim_7(&self) -> bool {
        *self == DCOFTRIM_A::DCOFTRIM_7
    }
}
#[doc = "Field `DCOFTRIM` writer - DCO frequency trim. Bit: 0"]
pub type DCOFTRIM_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL1_SPEC, u8, DCOFTRIM_A, 3, O>;
impl<'a, const O: u8> DCOFTRIM_W<'a, O> {
    #[doc = "DCO frequency trim: 0"]
    #[inline(always)]
    pub fn dcoftrim_0(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_0)
    }
    #[doc = "DCO frequency trim: 1"]
    #[inline(always)]
    pub fn dcoftrim_1(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_1)
    }
    #[doc = "DCO frequency trim: 2"]
    #[inline(always)]
    pub fn dcoftrim_2(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_2)
    }
    #[doc = "DCO frequency trim: 3"]
    #[inline(always)]
    pub fn dcoftrim_3(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_3)
    }
    #[doc = "DCO frequency trim: 4"]
    #[inline(always)]
    pub fn dcoftrim_4(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_4)
    }
    #[doc = "DCO frequency trim: 5"]
    #[inline(always)]
    pub fn dcoftrim_5(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_5)
    }
    #[doc = "DCO frequency trim: 6"]
    #[inline(always)]
    pub fn dcoftrim_6(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_6)
    }
    #[doc = "DCO frequency trim: 7"]
    #[inline(always)]
    pub fn dcoftrim_7(self) -> &'a mut W {
        self.variant(DCOFTRIM_A::DCOFTRIM_7)
    }
}
#[doc = "Field `DCOFTRIMEN` reader - DCO frequency trim enable"]
pub type DCOFTRIMEN_R = crate::BitReader<bool>;
#[doc = "Field `DCOFTRIMEN` writer - DCO frequency trim enable"]
pub type DCOFTRIMEN_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL1_SPEC, bool, O>;
impl R {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    pub fn dismod(&self) -> DISMOD_R {
        DISMOD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline(always)]
    pub fn dcorsel(&self) -> DCORSEL_R {
        DCORSEL_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline(always)]
    pub fn dcoftrim(&self) -> DCOFTRIM_R {
        DCOFTRIM_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline(always)]
    pub fn dcoftrimen(&self) -> DCOFTRIMEN_R {
        DCOFTRIMEN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 0 - Disable Modulation"]
    #[inline(always)]
    pub fn dismod(&mut self) -> DISMOD_W<0> {
        DISMOD_W::new(self)
    }
    #[doc = "Bits 1:3 - DCO frequency range select Bit: 0"]
    #[inline(always)]
    pub fn dcorsel(&mut self) -> DCORSEL_W<1> {
        DCORSEL_W::new(self)
    }
    #[doc = "Bits 4:6 - DCO frequency trim. Bit: 0"]
    #[inline(always)]
    pub fn dcoftrim(&mut self) -> DCOFTRIM_W<4> {
        DCOFTRIM_W::new(self)
    }
    #[doc = "Bit 7 - DCO frequency trim enable"]
    #[inline(always)]
    pub fn dcoftrimen(&mut self) -> DCOFTRIMEN_W<7> {
        DCOFTRIMEN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 1\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl1](index.html) module"]
pub struct CSCTL1_SPEC;
impl crate::RegisterSpec for CSCTL1_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl1::R](R) reader structure"]
impl crate::Readable for CSCTL1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl1::W](W) writer structure"]
impl crate::Writable for CSCTL1_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL1 to value 0"]
impl crate::Resettable for CSCTL1_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
