#[doc = "Register `IO_AHB_MATRIX2AC` reader"]
pub type R = crate::R<IoAhbMatrix2acSpec>;
#[doc = "Register `IO_AHB_MATRIX2AC` writer"]
pub type W = crate::W<IoAhbMatrix2acSpec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:31 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new((self.bits >> 9) & 0x007f_ffff)
    }
}
impl W {}
#[doc = "AHBM2AC Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2ac::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2ac::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix2acSpec;
impl crate::RegisterSpec for IoAhbMatrix2acSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix2ac::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix2acSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix2ac::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix2acSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX2AC to value 0"]
impl crate::Resettable for IoAhbMatrix2acSpec {}
