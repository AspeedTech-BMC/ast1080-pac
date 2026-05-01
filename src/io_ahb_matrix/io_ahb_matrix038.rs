#[doc = "Register `IO_AHB_MATRIX038` reader"]
pub type R = crate::R<IoAhbMatrix038Spec>;
#[doc = "Register `IO_AHB_MATRIX038` writer"]
pub type W = crate::W<IoAhbMatrix038Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 16:31 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
impl W {}
#[doc = "AHBM038 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix038Spec;
impl crate::RegisterSpec for IoAhbMatrix038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix038::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix038Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix038::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX038 to value 0"]
impl crate::Resettable for IoAhbMatrix038Spec {}
