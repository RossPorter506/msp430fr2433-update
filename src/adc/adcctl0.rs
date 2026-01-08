#[doc = "Register `ADCCTL0` reader"]
pub struct R(crate::R<ADCCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<ADCCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<ADCCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<ADCCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `ADCCTL0` writer"]
pub struct W(crate::W<ADCCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<ADCCTL0_SPEC>;
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
impl From<crate::W<ADCCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<ADCCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `ADCSC` reader - ADC Start Conversion"]
pub type ADCSC_R = crate::BitReader<bool>;
#[doc = "Field `ADCSC` writer - ADC Start Conversion"]
pub type ADCSC_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL0_SPEC, bool, O>;
#[doc = "Field `ADCENC` reader - ADC Enable Conversion"]
pub type ADCENC_R = crate::BitReader<bool>;
#[doc = "Field `ADCENC` writer - ADC Enable Conversion"]
pub type ADCENC_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL0_SPEC, bool, O>;
#[doc = "Field `ADCON` reader - ADC On/enable"]
pub type ADCON_R = crate::BitReader<bool>;
#[doc = "Field `ADCON` writer - ADC On/enable"]
pub type ADCON_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL0_SPEC, bool, O>;
#[doc = "Field `ADCMSC` reader - ADC Multiple SampleConversion"]
pub type ADCMSC_R = crate::BitReader<bool>;
#[doc = "Field `ADCMSC` writer - ADC Multiple SampleConversion"]
pub type ADCMSC_W<'a, const O: u8> = crate::BitWriter<'a, u16, ADCCTL0_SPEC, bool, O>;
#[doc = "Field `ADCSHT` reader - ADC Sample Hold Select Bit: 0"]
pub type ADCSHT_R = crate::FieldReader<u8, ADCSHT_A>;
#[doc = "ADC Sample Hold Select Bit: 0\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ADCSHT_A {
    #[doc = "0: ADC Sample Hold Select 0"]
    ADCSHT_0 = 0,
    #[doc = "1: ADC Sample Hold Select 1"]
    ADCSHT_1 = 1,
    #[doc = "2: ADC Sample Hold Select 2"]
    ADCSHT_2 = 2,
    #[doc = "3: ADC Sample Hold Select 3"]
    ADCSHT_3 = 3,
    #[doc = "4: ADC Sample Hold Select 4"]
    ADCSHT_4 = 4,
    #[doc = "5: ADC Sample Hold Select 5"]
    ADCSHT_5 = 5,
    #[doc = "6: ADC Sample Hold Select 6"]
    ADCSHT_6 = 6,
    #[doc = "7: ADC Sample Hold Select 7"]
    ADCSHT_7 = 7,
    #[doc = "8: ADC Sample Hold Select 8"]
    ADCSHT_8 = 8,
    #[doc = "9: ADC Sample Hold Select 9"]
    ADCSHT_9 = 9,
    #[doc = "10: ADC Sample Hold Select 10"]
    ADCSHT_10 = 10,
    #[doc = "11: ADC Sample Hold Select 11"]
    ADCSHT_11 = 11,
    #[doc = "12: ADC Sample Hold Select 12"]
    ADCSHT_12 = 12,
    #[doc = "13: ADC Sample Hold Select 13"]
    ADCSHT_13 = 13,
    #[doc = "14: ADC Sample Hold Select 14"]
    ADCSHT_14 = 14,
    #[doc = "15: ADC Sample Hold Select 15"]
    ADCSHT_15 = 15,
}
impl From<ADCSHT_A> for u8 {
    #[inline(always)]
    fn from(variant: ADCSHT_A) -> Self {
        variant as _
    }
}
impl ADCSHT_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> ADCSHT_A {
        match self.bits {
            0 => ADCSHT_A::ADCSHT_0,
            1 => ADCSHT_A::ADCSHT_1,
            2 => ADCSHT_A::ADCSHT_2,
            3 => ADCSHT_A::ADCSHT_3,
            4 => ADCSHT_A::ADCSHT_4,
            5 => ADCSHT_A::ADCSHT_5,
            6 => ADCSHT_A::ADCSHT_6,
            7 => ADCSHT_A::ADCSHT_7,
            8 => ADCSHT_A::ADCSHT_8,
            9 => ADCSHT_A::ADCSHT_9,
            10 => ADCSHT_A::ADCSHT_10,
            11 => ADCSHT_A::ADCSHT_11,
            12 => ADCSHT_A::ADCSHT_12,
            13 => ADCSHT_A::ADCSHT_13,
            14 => ADCSHT_A::ADCSHT_14,
            15 => ADCSHT_A::ADCSHT_15,
            _ => unreachable!(),
        }
    }
    #[doc = "Checks if the value of the field is `ADCSHT_0`"]
    #[inline(always)]
    pub fn is_adcsht_0(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_0
    }
    #[doc = "Checks if the value of the field is `ADCSHT_1`"]
    #[inline(always)]
    pub fn is_adcsht_1(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_1
    }
    #[doc = "Checks if the value of the field is `ADCSHT_2`"]
    #[inline(always)]
    pub fn is_adcsht_2(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_2
    }
    #[doc = "Checks if the value of the field is `ADCSHT_3`"]
    #[inline(always)]
    pub fn is_adcsht_3(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_3
    }
    #[doc = "Checks if the value of the field is `ADCSHT_4`"]
    #[inline(always)]
    pub fn is_adcsht_4(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_4
    }
    #[doc = "Checks if the value of the field is `ADCSHT_5`"]
    #[inline(always)]
    pub fn is_adcsht_5(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_5
    }
    #[doc = "Checks if the value of the field is `ADCSHT_6`"]
    #[inline(always)]
    pub fn is_adcsht_6(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_6
    }
    #[doc = "Checks if the value of the field is `ADCSHT_7`"]
    #[inline(always)]
    pub fn is_adcsht_7(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_7
    }
    #[doc = "Checks if the value of the field is `ADCSHT_8`"]
    #[inline(always)]
    pub fn is_adcsht_8(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_8
    }
    #[doc = "Checks if the value of the field is `ADCSHT_9`"]
    #[inline(always)]
    pub fn is_adcsht_9(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_9
    }
    #[doc = "Checks if the value of the field is `ADCSHT_10`"]
    #[inline(always)]
    pub fn is_adcsht_10(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_10
    }
    #[doc = "Checks if the value of the field is `ADCSHT_11`"]
    #[inline(always)]
    pub fn is_adcsht_11(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_11
    }
    #[doc = "Checks if the value of the field is `ADCSHT_12`"]
    #[inline(always)]
    pub fn is_adcsht_12(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_12
    }
    #[doc = "Checks if the value of the field is `ADCSHT_13`"]
    #[inline(always)]
    pub fn is_adcsht_13(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_13
    }
    #[doc = "Checks if the value of the field is `ADCSHT_14`"]
    #[inline(always)]
    pub fn is_adcsht_14(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_14
    }
    #[doc = "Checks if the value of the field is `ADCSHT_15`"]
    #[inline(always)]
    pub fn is_adcsht_15(&self) -> bool {
        *self == ADCSHT_A::ADCSHT_15
    }
}
#[doc = "Field `ADCSHT` writer - ADC Sample Hold Select Bit: 0"]
pub type ADCSHT_W<'a, const O: u8> =
    crate::FieldWriterSafe<'a, u16, ADCCTL0_SPEC, u8, ADCSHT_A, 4, O>;
impl<'a, const O: u8> ADCSHT_W<'a, O> {
    #[doc = "ADC Sample Hold Select 0"]
    #[inline(always)]
    pub fn adcsht_0(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_0)
    }
    #[doc = "ADC Sample Hold Select 1"]
    #[inline(always)]
    pub fn adcsht_1(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_1)
    }
    #[doc = "ADC Sample Hold Select 2"]
    #[inline(always)]
    pub fn adcsht_2(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_2)
    }
    #[doc = "ADC Sample Hold Select 3"]
    #[inline(always)]
    pub fn adcsht_3(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_3)
    }
    #[doc = "ADC Sample Hold Select 4"]
    #[inline(always)]
    pub fn adcsht_4(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_4)
    }
    #[doc = "ADC Sample Hold Select 5"]
    #[inline(always)]
    pub fn adcsht_5(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_5)
    }
    #[doc = "ADC Sample Hold Select 6"]
    #[inline(always)]
    pub fn adcsht_6(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_6)
    }
    #[doc = "ADC Sample Hold Select 7"]
    #[inline(always)]
    pub fn adcsht_7(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_7)
    }
    #[doc = "ADC Sample Hold Select 8"]
    #[inline(always)]
    pub fn adcsht_8(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_8)
    }
    #[doc = "ADC Sample Hold Select 9"]
    #[inline(always)]
    pub fn adcsht_9(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_9)
    }
    #[doc = "ADC Sample Hold Select 10"]
    #[inline(always)]
    pub fn adcsht_10(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_10)
    }
    #[doc = "ADC Sample Hold Select 11"]
    #[inline(always)]
    pub fn adcsht_11(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_11)
    }
    #[doc = "ADC Sample Hold Select 12"]
    #[inline(always)]
    pub fn adcsht_12(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_12)
    }
    #[doc = "ADC Sample Hold Select 13"]
    #[inline(always)]
    pub fn adcsht_13(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_13)
    }
    #[doc = "ADC Sample Hold Select 14"]
    #[inline(always)]
    pub fn adcsht_14(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_14)
    }
    #[doc = "ADC Sample Hold Select 15"]
    #[inline(always)]
    pub fn adcsht_15(self) -> &'a mut W {
        self.variant(ADCSHT_A::ADCSHT_15)
    }
}
impl R {
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline(always)]
    pub fn adcsc(&self) -> ADCSC_R {
        ADCSC_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline(always)]
    pub fn adcenc(&self) -> ADCENC_R {
        ADCENC_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline(always)]
    pub fn adcon(&self) -> ADCON_R {
        ADCON_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline(always)]
    pub fn adcmsc(&self) -> ADCMSC_R {
        ADCMSC_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adcsht(&self) -> ADCSHT_R {
        ADCSHT_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - ADC Start Conversion"]
    #[inline(always)]
    pub fn adcsc(&mut self) -> ADCSC_W<0> {
        ADCSC_W::new(self)
    }
    #[doc = "Bit 1 - ADC Enable Conversion"]
    #[inline(always)]
    pub fn adcenc(&mut self) -> ADCENC_W<1> {
        ADCENC_W::new(self)
    }
    #[doc = "Bit 4 - ADC On/enable"]
    #[inline(always)]
    pub fn adcon(&mut self) -> ADCON_W<4> {
        ADCON_W::new(self)
    }
    #[doc = "Bit 7 - ADC Multiple SampleConversion"]
    #[inline(always)]
    pub fn adcmsc(&mut self) -> ADCMSC_W<7> {
        ADCMSC_W::new(self)
    }
    #[doc = "Bits 8:11 - ADC Sample Hold Select Bit: 0"]
    #[inline(always)]
    pub fn adcsht(&mut self) -> ADCSHT_W<8> {
        ADCSHT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "ADC Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [adcctl0](index.html) module"]
pub struct ADCCTL0_SPEC;
impl crate::RegisterSpec for ADCCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [adcctl0::R](R) reader structure"]
impl crate::Readable for ADCCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [adcctl0::W](W) writer structure"]
impl crate::Writable for ADCCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets ADCCTL0 to value 0"]
impl crate::Resettable for ADCCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
