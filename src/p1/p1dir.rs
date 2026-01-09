#[doc = "Register `P1DIR` reader"]
pub struct R(crate::R<P1DIR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<P1DIR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<P1DIR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<P1DIR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `P1DIR` writer"]
pub struct W(crate::W<P1DIR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<P1DIR_SPEC>;
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
impl From<crate::W<P1DIR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<P1DIR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `P1DIR` reader - P1DIR"]
pub type P1DIR_R = crate::FieldReader<u8, u8>;
#[doc = "Field `P1DIR` writer - P1DIR"]
pub type P1DIR_W<'a, const O: u8> = crate::FieldWriterSafe<'a, u8, P1DIR_SPEC, u8, u8, 8, O>;
impl R {
    #[doc = "Bits 0:7 - P1DIR"]
    #[inline(always)]
    pub fn p1dir(&self) -> P1DIR_R {
        P1DIR_R::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:7 - P1DIR"]
    #[inline(always)]
    pub fn p1dir(&mut self) -> P1DIR_W<0> {
        P1DIR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub fn bits(&mut self, bits: u8) -> &mut Self {
        unsafe { self.0.bits(bits) };
        self
    }
}
#[doc = "Port 1 Direction register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [p1dir](index.html) module"]
pub struct P1DIR_SPEC;
impl crate::RegisterSpec for P1DIR_SPEC {
    type Ux = u8;
}
#[doc = "`read()` method returns [p1dir::R](R) reader structure"]
impl crate::Readable for P1DIR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [p1dir::W](W) writer structure"]
impl crate::Writable for P1DIR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets P1DIR to value 0"]
impl crate::Resettable for P1DIR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
