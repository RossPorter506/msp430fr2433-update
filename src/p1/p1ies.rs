#[doc = "Register `P1IES` reader"]
pub struct R(crate::R<P1IES_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1IES_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1IES_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1IES_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1IES` writer"]
pub struct W(crate::W<P1IES_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1IES_SPEC>;
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
impl From<crate::W<P1IES_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1IES_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1IES` reader - P1IES"]
pub type P1IES_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P1IES` writer - P1IES"]
pub type P1IES_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, P1IES_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - P1IES"]
    #[inline(always)]
    pub fn p1ies(&self) -> P1IES_R {
        P1IES_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1IES"]
    #[inline(always)]
    pub fn p1ies(&mut self) -> P1IES_W<0> {
        P1IES_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Port 1 Interrupt Edge Select register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1ies](index.html) module"]
pub struct P1IES_SPEC;
impl crate::RegisterSpec for P1IES_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1ies::R](R) reader structure"]
impl crate::Readable for P1IES_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1ies::W](W) writer structure"]
impl crate::Writable for P1IES_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1IES to value 0"]
impl crate::Resettable for P1IES_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
