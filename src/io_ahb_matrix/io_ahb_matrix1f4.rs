#[doc = "Register `IO_AHB_MATRIX1F4` reader"]
pub type R = crate::R<IoAhbMatrix1f4Spec>;
#[doc = "Register `IO_AHB_MATRIX1F4` writer"]
pub type W = crate::W<IoAhbMatrix1f4Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 6:31 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new((self.bits >> 6) & 0x03ff_ffff)
    }
}
impl W {}
#[doc = "AHBM1F4 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix1f4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix1f4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix1f4Spec;
impl crate::RegisterSpec for IoAhbMatrix1f4Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix1f4::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix1f4Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix1f4::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix1f4Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX1F4 to value 0x08"]
impl crate::Resettable for IoAhbMatrix1f4Spec {
    const RESET_VALUE: u32 = 0x08;
}
