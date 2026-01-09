#[doc = "Register `CSCTL4` reader"]
pub struct R(crate::R<CSCTL4_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CSCTL4_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CSCTL4_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CSCTL4_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CSCTL4` writer"]
pub struct W(crate::W<CSCTL4_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CSCTL4_SPEC>;
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
impl From<crate::W<CSCTL4_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CSCTL4_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SELMS` reader - MCLK and SMCLK Source Select Bit: 0"]
pub type SELMS_R = crate::FieldReader<u8, SELMS_A>;
#[doc = "MCLK and SMCLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum SELMS_A {
    #[doc = "0: DCO CLKDIV"]
    DCOCLKDIV = 0,
    #[doc = "1: REFOCLK"]
    REFOCLK = 1,
    #[doc = "2: XT1CLK"]
    XT1CLK = 2,
    #[doc = "3: VLOCLK"]
    VLOCLK = 3,
}
impl From<SELMS_A> for u8 {
    #[inline(always)]
    fn from(variant: SELMS_A) -> Self {
        variant as _
    }
}
impl SELMS_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<SELMS_A> {
        match self.bits {
            0 => Some(SELMS_A::DCOCLKDIV),
            1 => Some(SELMS_A::REFOCLK),
            2 => Some(SELMS_A::XT1CLK),
            3 => Some(SELMS_A::VLOCLK),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `DCOCLKDIV`"]
    #[inline(always)]
    pub fn is_dcoclkdiv(&self) -> bool {
        *self == SELMS_A::DCOCLKDIV
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELMS_A::REFOCLK
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELMS_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `VLOCLK`"]
    #[inline(always)]
    pub fn is_vloclk(&self) -> bool {
        *self == SELMS_A::VLOCLK
    }
}
#[doc = "Field `SELMS` writer - MCLK and SMCLK Source Select Bit: 0"]
pub type SELMS_W<'a, const O: u8> = crate::FieldWriter<'a, u16, CSCTL4_SPEC, u8, SELMS_A, 3, O>;
impl<'a, const O: u8> SELMS_W<'a, O> {
    #[doc = "DCO CLKDIV"]
    #[inline(always)]
    pub fn dcoclkdiv(self) -> &'a mut W {
        self.variant(SELMS_A::DCOCLKDIV)
    }
    #[doc = "REFOCLK"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELMS_A::REFOCLK)
    }
    #[doc = "XT1CLK"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELMS_A::XT1CLK)
    }
    #[doc = "VLOCLK"]
    #[inline(always)]
    pub fn vloclk(self) -> &'a mut W {
        self.variant(SELMS_A::VLOCLK)
    }
}
#[doc = "Field `SELA` reader - ACLK Source Select Bit: 0"]
pub type SELA_R = crate::BitReader<SELA_A>;
#[doc = "ACLK Source Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SELA_A {
    #[doc = "0: Source ACLK from XT1CLK with divider (no more than 40kHz)"]
    XT1CLK = 0,
    #[doc = "1: Source ACLK from the internal 32kHz clock source"]
    REFOCLK = 1,
}
impl From<SELA_A> for bool {
    #[inline(always)]
    fn from(variant: SELA_A) -> Self {
        variant as u8 != 0
    }
}
impl SELA_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> SELA_A {
        match self.bits {
            false => SELA_A::XT1CLK,
            true => SELA_A::REFOCLK,
        }
    }
    #[doc = "Checks if the value of the field is `XT1CLK`"]
    #[inline(always)]
    pub fn is_xt1clk(&self) -> bool {
        *self == SELA_A::XT1CLK
    }
    #[doc = "Checks if the value of the field is `REFOCLK`"]
    #[inline(always)]
    pub fn is_refoclk(&self) -> bool {
        *self == SELA_A::REFOCLK
    }
}
#[doc = "Field `SELA` writer - ACLK Source Select Bit: 0"]
pub type SELA_W<'a, const O: u8> = crate::BitWriter<'a, u16, CSCTL4_SPEC, SELA_A, O>;
impl<'a, const O: u8> SELA_W<'a, O> {
    #[doc = "Source ACLK from XT1CLK with divider (no more than 40kHz)"]
    #[inline(always)]
    pub fn xt1clk(self) -> &'a mut W {
        self.variant(SELA_A::XT1CLK)
    }
    #[doc = "Source ACLK from the internal 32kHz clock source"]
    #[inline(always)]
    pub fn refoclk(self) -> &'a mut W {
        self.variant(SELA_A::REFOCLK)
    }
}
impl R {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&self) -> SELMS_R {
        SELMS_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&self) -> SELA_R {
        SELA_R::new(((self.bits >> 8) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:2 - MCLK and SMCLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn selms(&mut self) -> SELMS_W<0> {
        SELMS_W::new(self)
    }
    #[doc = "Bit 8 - ACLK Source Select Bit: 0"]
    #[inline(always)]
    pub fn sela(&mut self) -> SELA_W<8> {
        SELA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "CS Control Register 4\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [csctl4](index.html) module"]
pub struct CSCTL4_SPEC;
impl crate::RegisterSpec for CSCTL4_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [csctl4::R](R) reader structure"]
impl crate::Readable for CSCTL4_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [csctl4::W](W) writer structure"]
impl crate::Writable for CSCTL4_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets CSCTL4 to value 0"]
impl crate::Resettable for CSCTL4_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
