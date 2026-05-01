#[doc = "Register `IO_AHB_MATRIX2A8` reader"]
pub type R = crate::R<IoAhbMatrix2a8Spec>;
#[doc = "Register `IO_AHB_MATRIX2A8` writer"]
pub type W = crate::W<IoAhbMatrix2a8Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `Reserved2` writer - Reserved"]
pub type Reserved2W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved4` writer - Reserved"]
pub type Reserved4W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader<u16>;
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
    #[doc = "Bits 9:15 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 9) & 0x7f) as u8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:31 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 17) & 0x7fff) as u16)
    }
}
impl W {
    #[doc = "Bit 8 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&mut self) -> Reserved2W<IoAhbMatrix2a8Spec> {
        Reserved2W::new(self, 8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&mut self) -> Reserved4W<IoAhbMatrix2a8Spec> {
        Reserved4W::new(self, 16)
    }
}
#[doc = "AHBM2A8 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix2a8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix2a8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix2a8Spec;
impl crate::RegisterSpec for IoAhbMatrix2a8Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix2a8::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix2a8Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix2a8::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix2a8Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX2A8 to value 0x0100"]
impl crate::Resettable for IoAhbMatrix2a8Spec {
    const RESET_VALUE: u32 = 0x0100;
}
