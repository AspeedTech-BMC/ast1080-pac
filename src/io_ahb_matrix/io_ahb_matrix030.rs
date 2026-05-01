#[doc = "Register `IO_AHB_MATRIX030` reader"]
pub type R = crate::R<IoAhbMatrix030Spec>;
#[doc = "Register `IO_AHB_MATRIX030` writer"]
pub type W = crate::W<IoAhbMatrix030Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:31 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 1) & 0x7fff_ffff)
    }
}
impl W {}
#[doc = "AHBM030 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix030::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix030::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix030Spec;
impl crate::RegisterSpec for IoAhbMatrix030Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix030::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix030Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix030::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix030Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX030 to value 0"]
impl crate::Resettable for IoAhbMatrix030Spec {}
