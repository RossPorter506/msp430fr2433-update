#[doc = "Register `CSCTL2` reader"]
pub struct R(crate::R<CSCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL2` writer"]
pub struct W(crate::W<CSCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL2_SPEC>;
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
impl From<crate::W<CSCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLLN0` reader - FLL Multipier Bit : 0"]
pub type FLLN0_R = crate::BitReader<bool>;
#[doc = "Field `FLLN0` writer - FLL Multipier Bit : 0"]
pub type FLLN0_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN1` reader - FLL Multipier Bit : 1"]
pub type FLLN1_R = crate::BitReader<bool>;
#[doc = "Field `FLLN1` writer - FLL Multipier Bit : 1"]
pub type FLLN1_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN2` reader - FLL Multipier Bit : 2"]
pub type FLLN2_R = crate::BitReader<bool>;
#[doc = "Field `FLLN2` writer - FLL Multipier Bit : 2"]
pub type FLLN2_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN3` reader - FLL Multipier Bit : 3"]
pub type FLLN3_R = crate::BitReader<bool>;
#[doc = "Field `FLLN3` writer - FLL Multipier Bit : 3"]
pub type FLLN3_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN4` reader - FLL Multipier Bit : 4"]
pub type FLLN4_R = crate::BitReader<bool>;
#[doc = "Field `FLLN4` writer - FLL Multipier Bit : 4"]
pub type FLLN4_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN5` reader - FLL Multipier Bit : 5"]
pub type FLLN5_R = crate::BitReader<bool>;
#[doc = "Field `FLLN5` writer - FLL Multipier Bit : 5"]
pub type FLLN5_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN6` reader - FLL Multipier Bit : 6"]
pub type FLLN6_R = crate::BitReader<bool>;
#[doc = "Field `FLLN6` writer - FLL Multipier Bit : 6"]
pub type FLLN6_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN7` reader - FLL Multipier Bit : 7"]
pub type FLLN7_R = crate::BitReader<bool>;
#[doc = "Field `FLLN7` writer - FLL Multipier Bit : 7"]
pub type FLLN7_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN8` reader - FLL Multipier Bit : 8"]
pub type FLLN8_R = crate::BitReader<bool>;
#[doc = "Field `FLLN8` writer - FLL Multipier Bit : 8"]
pub type FLLN8_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLN9` reader - FLL Multipier Bit : 9"]
pub type FLLN9_R = crate::BitReader<bool>;
#[doc = "Field `FLLN9` writer - FLL Multipier Bit : 9"]
pub type FLLN9_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL2_SPEC, bool, O>;
#[doc = "Field `FLLD` reader - Loop Divider Bit : 0"]
pub type FLLD_R = crate::FieldReader<u8, FLLD_A>;
#[doc = "Loop Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLD_A {
    #[doc = "0: Multiply selected loop frequency by 1"]
    _1 = 0,
    #[doc = "1: Multiply selected loop frequency by 2"]
    _2 = 1,
    #[doc = "2: Multiply selected loop frequency by 4"]
    _4 = 2,
    #[doc = "3: Multiply selected loop frequency by 8"]
    _8 = 3,
    #[doc = "4: Multiply selected loop frequency by 16"]
    _16 = 4,
    #[doc = "5: Multiply selected loop frequency by 32"]
    _32 = 5,
}
impl From<FLLD_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLD_A) -> Self {
        variant as _
    }
}
impl FLLD_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<FLLD_A> {
        match self.bits {
            0 => Some(FLLD_A::_1),
            1 => Some(FLLD_A::_2),
            2 => Some(FLLD_A::_4),
            3 => Some(FLLD_A::_8),
            4 => Some(FLLD_A::_16),
            5 => Some(FLLD_A::_32),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `_1`"]
    #[inline(always)]
    pub fn is_1(&self) -> bool {
        *self == FLLD_A::_1
    }
    #[doc = "Checks if the value of the field is `_2`"]
    #[inline(always)]
    pub fn is_2(&self) -> bool {
        *self == FLLD_A::_2
    }
    #[doc = "Checks if the value of the field is `_4`"]
    #[inline(always)]
    pub fn is_4(&self) -> bool {
        *self == FLLD_A::_4
    }
    #[doc = "Checks if the value of the field is `_8`"]
    #[inline(always)]
    pub fn is_8(&self) -> bool {
        *self == FLLD_A::_8
    }
    #[doc = "Checks if the value of the field is `_16`"]
    #[inline(always)]
    pub fn is_16(&self) -> bool {
        *self == FLLD_A::_16
    }
    #[doc = "Checks if the value of the field is `_32`"]
    #[inline(always)]
    pub fn is_32(&self) -> bool {
        *self == FLLD_A::_32
    }
}
#[doc = "Field `FLLD` writer - Loop Divider Bit : 0"]
pub type FLLD_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL2_SPEC, u8, FLLD_A, 3, O>;
impl<'a, const O: u8> FLLD_W<'a, O> {
    #[doc = "Multiply selected loop frequency by 1"]
    #[inline(always)]
    pub fn _1(self) -> &'a mut W {
        self.variant(FLLD_A::_1)
    }
    #[doc = "Multiply selected loop frequency by 2"]
    #[inline(always)]
    pub fn _2(self) -> &'a mut W {
        self.variant(FLLD_A::_2)
    }
    #[doc = "Multiply selected loop frequency by 4"]
    #[inline(always)]
    pub fn _4(self) -> &'a mut W {
        self.variant(FLLD_A::_4)
    }
    #[doc = "Multiply selected loop frequency by 8"]
    #[inline(always)]
    pub fn _8(self) -> &'a mut W {
        self.variant(FLLD_A::_8)
    }
    #[doc = "Multiply selected loop frequency by 16"]
    #[inline(always)]
    pub fn _16(self) -> &'a mut W {
        self.variant(FLLD_A::_16)
    }
    #[doc = "Multiply selected loop frequency by 32"]
    #[inline(always)]
    pub fn _32(self) -> &'a mut W {
        self.variant(FLLD_A::_32)
    }
}
impl R {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln0(&self) -> FLLN0_R {
        FLLN0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    pub fn flln1(&self) -> FLLN1_R {
        FLLN1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    pub fn flln2(&self) -> FLLN2_R {
        FLLN2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    pub fn flln3(&self) -> FLLN3_R {
        FLLN3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    pub fn flln4(&self) -> FLLN4_R {
        FLLN4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    pub fn flln5(&self) -> FLLN5_R {
        FLLN5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    pub fn flln6(&self) -> FLLN6_R {
        FLLN6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    pub fn flln7(&self) -> FLLN7_R {
        FLLN7_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    pub fn flln8(&self) -> FLLN8_R {
        FLLN8_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    pub fn flln9(&self) -> FLLN9_R {
        FLLN9_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&self) -> FLLD_R {
        FLLD_R::new(((self.bits >> 12) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - FLL Multipier Bit : 0"]
    #[inline(always)]
    pub fn flln0(&mut self) -> FLLN0_W<0> {
        FLLN0_W::new(self)
    }
    #[doc = "Bit 1 - FLL Multipier Bit : 1"]
    #[inline(always)]
    pub fn flln1(&mut self) -> FLLN1_W<1> {
        FLLN1_W::new(self)
    }
    #[doc = "Bit 2 - FLL Multipier Bit : 2"]
    #[inline(always)]
    pub fn flln2(&mut self) -> FLLN2_W<2> {
        FLLN2_W::new(self)
    }
    #[doc = "Bit 3 - FLL Multipier Bit : 3"]
    #[inline(always)]
    pub fn flln3(&mut self) -> FLLN3_W<3> {
        FLLN3_W::new(self)
    }
    #[doc = "Bit 4 - FLL Multipier Bit : 4"]
    #[inline(always)]
    pub fn flln4(&mut self) -> FLLN4_W<4> {
        FLLN4_W::new(self)
    }
    #[doc = "Bit 5 - FLL Multipier Bit : 5"]
    #[inline(always)]
    pub fn flln5(&mut self) -> FLLN5_W<5> {
        FLLN5_W::new(self)
    }
    #[doc = "Bit 6 - FLL Multipier Bit : 6"]
    #[inline(always)]
    pub fn flln6(&mut self) -> FLLN6_W<6> {
        FLLN6_W::new(self)
    }
    #[doc = "Bit 7 - FLL Multipier Bit : 7"]
    #[inline(always)]
    pub fn flln7(&mut self) -> FLLN7_W<7> {
        FLLN7_W::new(self)
    }
    #[doc = "Bit 8 - FLL Multipier Bit : 8"]
    #[inline(always)]
    pub fn flln8(&mut self) -> FLLN8_W<8> {
        FLLN8_W::new(self)
    }
    #[doc = "Bit 9 - FLL Multipier Bit : 9"]
    #[inline(always)]
    pub fn flln9(&mut self) -> FLLN9_W<9> {
        FLLN9_W::new(self)
    }
    #[doc = "Bits 12:14 - Loop Divider Bit : 0"]
    #[inline(always)]
    pub fn flld(&mut self) -> FLLD_W<12> {
        FLLD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl2](index.html) module"]
pub struct CSCTL2_SPEC;
impl crate::RegisterSpec for CSCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl2::R](R) reader structure"]
impl crate::Readable for CSCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl2::W](W) writer structure"]
impl crate::Writable for CSCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL2 to value 0"]
impl crate::Resettable for CSCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
