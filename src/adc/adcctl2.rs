#[doc = "Register `ADCCTL2` reader"]
pub struct R(crate::R<ADCCTL2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTL2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTL2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTL2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTL2` writer"]
pub struct W(crate::W<ADCCTL2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTL2_SPEC>;
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
impl From<crate::W<ADCCTL2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTL2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSR` reader - ADC Sampling Rate"]
pub type ADCSR_R = crate::BitReader<bool>;
#[doc = "Field `ADCSR` writer - ADC Sampling Rate"]
pub type ADCSR_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL2_SPEC, bool, O>;
#[doc = "Field `ADCDF` reader - ADC Data Format"]
pub type ADCDF_R = crate::BitReader<bool>;
#[doc = "Field `ADCDF` writer - ADC Data Format"]
pub type ADCDF_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL2_SPEC, bool, O>;
#[doc = "Field `ADCRES` reader - ADC Resolution"]
pub type ADCRES_R = crate::FieldReader<u8, ADCRES_A>;
#[doc = "ADC Resolution\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCRES_A {
    #[doc = "0: 8 bit"]
    ADCRES_0 = 0,
    #[doc = "1: 10 bit"]
    ADCRES_1 = 1,
    #[doc = "2: Reserved"]
    ADCRES_2 = 2,
    #[doc = "3: Reserved"]
    ADCRES_3 = 3,
}
impl From<ADCRES_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCRES_A) -> Self {
        variant as _
    }
}
impl ADCRES_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCRES_A {
        match self.bits {
            0 => ADCRES_A::ADCRES_0,
            1 => ADCRES_A::ADCRES_1,
            2 => ADCRES_A::ADCRES_2,
            3 => ADCRES_A::ADCRES_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCRES_0`"]
    #[inline(always)]
    pub fn is_adcres_0(&self) -> bool {
        *self == ADCRES_A::ADCRES_0
    }
    #[doc = "Checks if the value of the field is `ADCRES_1`"]
    #[inline(always)]
    pub fn is_adcres_1(&self) -> bool {
        *self == ADCRES_A::ADCRES_1
    }
    #[doc = "Checks if the value of the field is `ADCRES_2`"]
    #[inline(always)]
    pub fn is_adcres_2(&self) -> bool {
        *self == ADCRES_A::ADCRES_2
    }
    #[doc = "Checks if the value of the field is `ADCRES_3`"]
    #[inline(always)]
    pub fn is_adcres_3(&self) -> bool {
        *self == ADCRES_A::ADCRES_3
    }
}
#[doc = "Field `ADCRES` writer - ADC Resolution"]
pub type ADCRES_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCCTL2_SPEC, u8, ADCRES_A, 2, O>;
impl<'a, const O: u8> ADCRES_W<'a, O> {
    #[doc = "8 bit"]
    #[inline(always)]
    pub fn adcres_0(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_0)
    }
    #[doc = "10 bit"]
    #[inline(always)]
    pub fn adcres_1(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_1)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_2(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_2)
    }
    #[doc = "Reserved"]
    #[inline(always)]
    pub fn adcres_3(self) -> &'a mut W {
        self.variant(ADCRES_A::ADCRES_3)
    }
}
#[doc = "Field `ADCPDIV` reader - ADC predivider Bit: 0"]
pub type ADCPDIV_R = crate::FieldReader<u8, ADCPDIV_A>;
#[doc = "ADC predivider Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCPDIV_A {
    #[doc = "0: ADC predivider /1"]
    ADCPDIV_0 = 0,
    #[doc = "1: ADC predivider /2"]
    ADCPDIV_1 = 1,
    #[doc = "2: ADC predivider /64"]
    ADCPDIV_2 = 2,
    #[doc = "3: ADC predivider reserved"]
    ADCPDIV_3 = 3,
}
impl From<ADCPDIV_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCPDIV_A) -> Self {
        variant as _
    }
}
impl ADCPDIV_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCPDIV_A {
        match self.bits {
            0 => ADCPDIV_A::ADCPDIV_0,
            1 => ADCPDIV_A::ADCPDIV_1,
            2 => ADCPDIV_A::ADCPDIV_2,
            3 => ADCPDIV_A::ADCPDIV_3,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_0`"]
    #[inline(always)]
    pub fn is_adcpdiv_0(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_0
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_1`"]
    #[inline(always)]
    pub fn is_adcpdiv_1(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_1
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_2`"]
    #[inline(always)]
    pub fn is_adcpdiv_2(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_2
    }
    #[doc = "Checks if the value of the field is `ADCPDIV_3`"]
    #[inline(always)]
    pub fn is_adcpdiv_3(&self) -> bool {
        *self == ADCPDIV_A::ADCPDIV_3
    }
}
#[doc = "Field `ADCPDIV` writer - ADC predivider Bit: 0"]
pub type ADCPDIV_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCCTL2_SPEC, u8, ADCPDIV_A, 2, O>;
impl<'a, const O: u8> ADCPDIV_W<'a, O> {
    #[doc = "ADC predivider /1"]
    #[inline(always)]
    pub fn adcpdiv_0(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_0)
    }
    #[doc = "ADC predivider /2"]
    #[inline(always)]
    pub fn adcpdiv_1(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_1)
    }
    #[doc = "ADC predivider /64"]
    #[inline(always)]
    pub fn adcpdiv_2(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_2)
    }
    #[doc = "ADC predivider reserved"]
    #[inline(always)]
    pub fn adcpdiv_3(self) -> &'a mut W {
        self.variant(ADCPDIV_A::ADCPDIV_3)
    }
}
impl R {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    pub fn adcsr(&self) -> ADCSR_R {
        ADCSR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    pub fn adcdf(&self) -> ADCDF_R {
        ADCDF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    pub fn adcres(&self) -> ADCRES_R {
        ADCRES_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    pub fn adcpdiv(&self) -> ADCPDIV_R {
        ADCPDIV_R::new(((self.bits >> 8) & 3) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - ADC Sampling Rate"]
    #[inline(always)]
    pub fn adcsr(&mut self) -> ADCSR_W<2> {
        ADCSR_W::new(self)
    }
    #[doc = "Bit 3 - ADC Data Format"]
    #[inline(always)]
    pub fn adcdf(&mut self) -> ADCDF_W<3> {
        ADCDF_W::new(self)
    }
    #[doc = "Bits 4:5 - ADC Resolution"]
    #[inline(always)]
    pub fn adcres(&mut self) -> ADCRES_W<4> {
        ADCRES_W::new(self)
    }
    #[doc = "Bits 8:9 - ADC predivider Bit: 0"]
    #[inline(always)]
    pub fn adcpdiv(&mut self) -> ADCPDIV_W<8> {
        ADCPDIV_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control 2\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl2](index.html) module"]
pub struct ADCCTL2_SPEC;
impl crate::RegisterSpec for ADCCTL2_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcctl2::R](R) reader structure"]
impl crate::Readable for ADCCTL2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctl2::W](W) writer structure"]
impl crate::Writable for ADCCTL2_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCTL2 to value 0"]
impl crate::Resettable for ADCCTL2_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
