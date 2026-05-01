#[doc = "Register `IO_AHB_MATRIX04C` reader"]
pub type R = crate::R<IoAhbMatrix04cSpec>;
#[doc = "Register `IO_AHB_MATRIX04C` writer"]
pub type W = crate::W<IoAhbMatrix04cSpec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader<u16>;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 3:15 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1fff) as u16)
    }
    #[doc = "Bits 16:19 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 16) & 0x0f) as u8)
    }
    #[doc = "Bits 20:31 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 20) & 0x0fff) as u16)
    }
}
impl W {}
#[doc = "AHBM04C Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix04c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix04c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix04cSpec;
impl crate::RegisterSpec for IoAhbMatrix04cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix04c::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix04cSpec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix04c::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix04cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX04C to value 0"]
impl crate::Resettable for IoAhbMatrix04cSpec {}
