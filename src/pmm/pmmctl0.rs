#[doc = "Register `PMMCTL0` reader"]
pub struct R(crate::R<PMMCTL0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PMMCTL0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PMMCTL0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PMMCTL0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PMMCTL0` writer"]
pub struct W(crate::W<PMMCTL0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PMMCTL0_SPEC>;
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
impl From<crate::W<PMMCTL0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PMMCTL0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PMMSWBOR` reader - PMM Software BOR"]
pub type PMMSWBOR_R = crate::BitReader<bool>;
#[doc = "Field `PMMSWBOR` writer - PMM Software BOR"]
pub type PMMSWBOR_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, bool, O>;
#[doc = "Field `PMMSWPOR` reader - PMM Software POR"]
pub type PMMSWPOR_R = crate::BitReader<bool>;
#[doc = "Field `PMMSWPOR` writer - PMM Software POR"]
pub type PMMSWPOR_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, bool, O>;
#[doc = "Field `PMMREGOFF` reader - PMM Turn Regulator off"]
pub type PMMREGOFF_R = crate::BitReader<bool>;
#[doc = "Field `PMMREGOFF` writer - PMM Turn Regulator off"]
pub type PMMREGOFF_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, bool, O>;
#[doc = "Field `SVSHE` reader - SVS high side enable"]
pub type SVSHE_R = crate::BitReader<bool>;
#[doc = "Field `SVSHE` writer - SVS high side enable"]
pub type SVSHE_W<'a, const O: u8> = crate::BitWriter<'a, u16, PMMCTL0_SPEC, bool, O>;
#[doc = "Field `PMMPW` reader - PMM Password"]
pub type PMMPW_R = crate::FieldReader<u8, PMMPWR_A>;
#[doc = "PMM Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMMPWR_A {
    #[doc = "150: Values always reads from the PMMCTL0 register"]
    PASSWORD = 150,
}
impl From<PMMPWR_A> for u8 {
    #[inline(always)]
    fn from(variant: PMMPWR_A) -> Self {
        variant as _
    }
}
impl PMMPW_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub fn variant(&self) -> Option<PMMPWR_A> {
        match self.bits {
            150 => Some(PMMPWR_A::PASSWORD),
            _ => None,
        }
    }
    #[doc = "Checks if the value of the field is `PASSWORD`"]
    #[inline(always)]
    pub fn is_password(&self) -> bool {
        *self == PMMPWR_A::PASSWORD
    }
}
#[doc = "PMM Password\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum PMMPWW_AW {
    #[doc = "165: Values which must be written to the PMMCTL0 register"]
    PASSWORD = 165,
}
impl From<PMMPWW_AW> for u8 {
    #[inline(always)]
    fn from(variant: PMMPWW_AW) -> Self {
        variant as _
    }
}
#[doc = "Field `PMMPW` writer - PMM Password"]
pub type PMMPW_W<'a, const O: u8> = crate::FieldWriter<'a, u16, PMMCTL0_SPEC, u8, PMMPWW_AW, 8, O>;
impl<'a, const O: u8> PMMPW_W<'a, O> {
    #[doc = "Values which must be written to the PMMCTL0 register"]
    #[inline(always)]
    pub fn password(self) -> &'a mut W {
        self.variant(PMMPWW_AW::PASSWORD)
    }
}
impl R {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&self) -> PMMSWBOR_R {
        PMMSWBOR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&self) -> PMMSWPOR_R {
        PMMSWPOR_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&self) -> PMMREGOFF_R {
        PMMREGOFF_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&self) -> SVSHE_R {
        SVSHE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bits 8:15 - PMM Password"]
    #[inline(always)]
    pub fn pmmpw(&self) -> PMMPW_R {
        PMMPW_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 2 - PMM Software BOR"]
    #[inline(always)]
    pub fn pmmswbor(&mut self) -> PMMSWBOR_W<2> {
        PMMSWBOR_W::new(self)
    }
    #[doc = "Bit 3 - PMM Software POR"]
    #[inline(always)]
    pub fn pmmswpor(&mut self) -> PMMSWPOR_W<3> {
        PMMSWPOR_W::new(self)
    }
    #[doc = "Bit 4 - PMM Turn Regulator off"]
    #[inline(always)]
    pub fn pmmregoff(&mut self) -> PMMREGOFF_W<4> {
        PMMREGOFF_W::new(self)
    }
    #[doc = "Bit 6 - SVS high side enable"]
    #[inline(always)]
    pub fn svshe(&mut self) -> SVSHE_W<6> {
        SVSHE_W::new(self)
    }
    #[doc = "Bits 8:15 - PMM Password"]
    #[inline(always)]
    pub fn pmmpw(&mut self) -> PMMPW_W<8> {
        PMMPW_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u16) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "PMM Control 0\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pmmctl0](index.html) module"]
pub struct PMMCTL0_SPEC;
impl crate::RegisterSpec for PMMCTL0_SPEC {
    type Ux = u16;
}
#[doc = "`read()` method returns [pmmctl0::R](R) reader structure"]
impl crate::Readable for PMMCTL0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pmmctl0::W](W) writer structure"]
impl crate::Writable for PMMCTL0_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets PMMCTL0 to value 0"]
impl crate::Resettable for PMMCTL0_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
