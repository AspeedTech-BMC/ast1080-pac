#[doc = "Register `IO_AHB_MATRIX200` reader"]
pub type R = crate::R<IoAhbMatrix200Spec>;
#[doc = "Register `IO_AHB_MATRIX200` writer"]
pub type W = crate::W<IoAhbMatrix200Spec>;
#[doc = "Field `Reserved1` reader - Reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `Reserved2` reader - Reserved"]
pub type Reserved2R = crate::FieldReader;
#[doc = "Field `Reserved3` reader - Reserved"]
pub type Reserved3R = crate::FieldReader;
#[doc = "Field `Reserved4` reader - Reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `Reserved5` reader - Reserved"]
pub type Reserved5R = crate::FieldReader;
#[doc = "Field `Reserved6` reader - Reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `Reserved7` reader - Reserved"]
pub type Reserved7R = crate::FieldReader;
#[doc = "Field `Reserved8` reader - Reserved"]
pub type Reserved8R = crate::BitReader;
#[doc = "Field `Reserved8` writer - Reserved"]
pub type Reserved8W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved9` reader - Reserved"]
pub type Reserved9R = crate::FieldReader;
#[doc = "Field `Reserved10` reader - Reserved"]
pub type Reserved10R = crate::BitReader;
#[doc = "Field `Reserved10` writer - Reserved"]
pub type Reserved10W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved11` reader - Reserved"]
pub type Reserved11R = crate::FieldReader;
impl R {
    #[doc = "Bits 1:7 - Reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 1) & 0x7f) as u8)
    }
    #[doc = "Bits 8:11 - Reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - Reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - Reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - Reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - Reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:23 - Reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&self) -> Reserved8R {
        Reserved8R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bits 25:27 - Reserved"]
    #[inline(always)]
    pub fn reserved9(&self) -> Reserved9R {
        Reserved9R::new(((self.bits >> 25) & 7) as u8)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&self) -> Reserved10R {
        Reserved10R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bits 29:31 - Reserved"]
    #[inline(always)]
    pub fn reserved11(&self) -> Reserved11R {
        Reserved11R::new(((self.bits >> 29) & 7) as u8)
    }
}
impl W {
    #[doc = "Bit 24 - Reserved"]
    #[inline(always)]
    pub fn reserved8(&mut self) -> Reserved8W<IoAhbMatrix200Spec> {
        Reserved8W::new(self, 24)
    }
    #[doc = "Bit 28 - Reserved"]
    #[inline(always)]
    pub fn reserved10(&mut self) -> Reserved10W<IoAhbMatrix200Spec> {
        Reserved10W::new(self, 28)
    }
}
#[doc = "AHBM200 Register\n\nYou can [`read`](crate::Reg::read) this register and get [`io_ahb_matrix200::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`io_ahb_matrix200::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IoAhbMatrix200Spec;
impl crate::RegisterSpec for IoAhbMatrix200Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`io_ahb_matrix200::R`](R) reader structure"]
impl crate::Readable for IoAhbMatrix200Spec {}
#[doc = "`write(|w| ..)` method takes [`io_ahb_matrix200::W`](W) writer structure"]
impl crate::Writable for IoAhbMatrix200Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IO_AHB_MATRIX200 to value 0"]
impl crate::Resettable for IoAhbMatrix200Spec {}
