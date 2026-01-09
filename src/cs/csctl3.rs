#[doc = "Register `CSCTL3` reader"]
pub struct R(crate::R<CSCTL3_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL3_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL3_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL3_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL3` writer"]
pub struct W(crate::W<CSCTL3_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL3_SPEC>;
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
impl From<crate::W<CSCTL3_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL3_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `FLLREFDIV` reader - Reference Divider Bit : 0"]
pub type FLLREFDIV_R = crate::FieldReader<u8, FLLREFDIV_A>;
#[doc = "Reference Divider Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum FLLREFDIV_A {
    #[doc = "0: Reference Divider: f(LFCLK)/1"]
    FLLREFDIV_0 = 0,
    #[doc = "1: Reference Divider: f(LFCLK)/2"]
    FLLREFDIV_1 = 1,
    #[doc = "2: Reference Divider: f(LFCLK)/4"]
    FLLREFDIV_2 = 2,
    #[doc = "3: Reference Divider: f(LFCLK)/8"]
    FLLREFDIV_3 = 3,
    #[doc = "4: Reference Divider: f(LFCLK)/12"]
    FLLREFDIV_4 = 4,
    #[doc = "5: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_5 = 5,
    #[doc = "6: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_6 = 6,
    #[doc = "7: Reference Divider: f(LFCLK)/16"]
    FLLREFDIV_7 = 7,
}
impl From<FLLREFDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: FLLREFDIV_A) -> Self {
        variant as _
    }
}
impl FLLREFDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> FLLREFDIV_A {
        match self.bits {
            0 => FLLREFDIV_A::FLLREFDIV_0,
            1 => FLLREFDIV_A::FLLREFDIV_1,
            2 => FLLREFDIV_A::FLLREFDIV_2,
            3 => FLLREFDIV_A::FLLREFDIV_3,
            4 => FLLREFDIV_A::FLLREFDIV_4,
            5 => FLLREFDIV_A::FLLREFDIV_5,
            6 => FLLREFDIV_A::FLLREFDIV_6,
            7 => FLLREFDIV_A::FLLREFDIV_7,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_0`"]
    #[inline(always)]
    pub fn is_fllrefdiv_0(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_0
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_1`"]
    #[inline(always)]
    pub fn is_fllrefdiv_1(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_1
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_2`"]
    #[inline(always)]
    pub fn is_fllrefdiv_2(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_2
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_3`"]
    #[inline(always)]
    pub fn is_fllrefdiv_3(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_3
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_4`"]
    #[inline(always)]
    pub fn is_fllrefdiv_4(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_4
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_5`"]
    #[inline(always)]
    pub fn is_fllrefdiv_5(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_5
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_6`"]
    #[inline(always)]
    pub fn is_fllrefdiv_6(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_6
    }
    #[doc = "Checks if the value of the field is `FLLREFDIV_7`"]
    #[inline(always)]
    pub fn is_fllrefdiv_7(&self) -> bool {
        *self == FLLREFDIV_A::FLLREFDIV_7
    }
}
#[doc = "Field `FLLREFDIV` writer - Reference Divider Bit : 0"]
pub type FLLREFDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, CSCTL3_SPEC, u8, FLLREFDIV_A, 3, O>;
impl<'a, const O: u8> FLLREFDIV_W<'a, O> {
    #[doc = "Reference Divider: f(LFCLK)/1"]
    #[inline(always)]
    pub fn fllrefdiv_0(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_0)
    }
    #[doc = "Reference Divider: f(LFCLK)/2"]
    #[inline(always)]
    pub fn fllrefdiv_1(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_1)
    }
    #[doc = "Reference Divider: f(LFCLK)/4"]
    #[inline(always)]
    pub fn fllrefdiv_2(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_2)
    }
    #[doc = "Reference Divider: f(LFCLK)/8"]
    #[inline(always)]
    pub fn fllrefdiv_3(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_3)
    }
    #[doc = "Reference Divider: f(LFCLK)/12"]
    #[inline(always)]
    pub fn fllrefdiv_4(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_4)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_5(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_5)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_6(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_6)
    }
    #[doc = "Reference Divider: f(LFCLK)/16"]
    #[inline(always)]
    pub fn fllrefdiv_7(self) -> &'a mut W {
        self.variant(FLLREFDIV_A::FLLREFDIV_7)
    }
}
#[doc = "Field `SELREF` reader - FLL Reference Clock Select Bit : 0"]
pub type SELREF_R = crate::FieldReader<u8, SELREF_A>;
#[doc = "FLL Reference Clock Select Bit : 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELREF_A {
    #[doc = "0: XT1CLK"]
    XT1CLK = 0,
    #[doc = "1: REFOCLK"]
    REFOCLK = 1,
}
impl From<SELREF_A> for u8 {
    #[inline(always)]
    fn from(variant: SELREF_A) -> Self {
        variant as _
    }
}
impl SELREF_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELREF_A> {
        match self.bits {
            0 => Some(SELREF_A::XT1CLK),
            1 => Some(SELREF_A::REFOCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELREF_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELREF_A::REFOCLK
    }
}
#[doc = "Field `SELREF` writer - FLL Reference Clock Select Bit : 0"]
pub type SELREF_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL3_SPEC, u8, SELREF_A, 2, O>;
impl<'a, const O: u8> SELREF_W<'a, O> {
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELREF_A::XT1CLK)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELREF_A::REFOCLK)
    }
}
impl R {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    pub fn fllrefdiv(&self) -> FLLREFDIV_R {
        FLLREFDIV_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 4:5 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    pub fn selref(&self) -> SELREF_R {
        SELREF_R::new(((self.bits >> 4) & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - Reference Divider Bit : 0"]
    #[inline(always)]
    pub fn fllrefdiv(&mut self) -> FLLREFDIV_W<0> {
        FLLREFDIV_W::new(self)
    }
    #[doc = "Bits 4:5 - FLL Reference Clock Select Bit : 0"]
    #[inline(always)]
    pub fn selref(&mut self) -> SELREF_W<4> {
        SELREF_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 3\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl3](index.html) module"]
pub struct CSCTL3_SPEC;
impl crate::RegisterSpec for CSCTL3_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl3::R](R) reader structure"]
impl crate::Readable for CSCTL3_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl3::W](W) writer structure"]
impl crate::Writable for CSCTL3_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL3 to value 0"]
impl crate::Resettable for CSCTL3_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
