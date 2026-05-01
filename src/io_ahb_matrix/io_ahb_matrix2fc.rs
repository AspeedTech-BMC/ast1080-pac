#[doc = "Register `IO_AHB_MATRIX2FC` reader"]
pub type R = crate::R<IoAhbMatrix2fcSpec>;
#[doc = "Register `IO_AHB_MATRIX2FC` writer"]
pub type W = crate::W<IoAhbMatrix2fcSpec>;
impl W {}
#[doc = "AHBM2FC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2fc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2fc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix2fcSpec;
impl crate::RegisterSpec for IoAhbMatrix2fcSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix2fc::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix2fcSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix2fc::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix2fcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX2FC to value 0"]
impl crate::Resettable for IoAhbMatrix2fcSpec {}
