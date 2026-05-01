#[doc = "Register `I3CCONTROL044` reader"]
pub type R = crate::R<I3ccontrol044Spec>;
#[doc = "Register `I3CCONTROL044` writer"]
pub type W = crate::W<I3ccontrol044Spec>;
#[doc = "Field `REGAUTOCMDDEV8` reader - REG_AUTOCMD_DEV_8"]
pub type Regautocmddev8R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV8` writer - REG_AUTOCMD_DEV_8"]
pub type Regautocmddev8W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV9` reader - REG_AUTOCMD_DEV_9"]
pub type Regautocmddev9R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV9` writer - REG_AUTOCMD_DEV_9"]
pub type Regautocmddev9W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV10` reader - REG_AUTOCMD_DEV_10"]
pub type Regautocmddev10R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV10` writer - REG_AUTOCMD_DEV_10"]
pub type Regautocmddev10W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV11` reader - REG_AUTOCMD_DEV_11"]
pub type Regautocmddev11R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV11` writer - REG_AUTOCMD_DEV_11"]
pub type Regautocmddev11W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV12` reader - REG_AUTOCMD_DEV_12"]
pub type Regautocmddev12R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV12` writer - REG_AUTOCMD_DEV_12"]
pub type Regautocmddev12W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV13` reader - REG_AUTOCMD_DEV_13"]
pub type Regautocmddev13R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV13` writer - REG_AUTOCMD_DEV_13"]
pub type Regautocmddev13W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV14` reader - REG_AUTOCMD_DEV_14"]
pub type Regautocmddev14R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV14` writer - REG_AUTOCMD_DEV_14"]
pub type Regautocmddev14W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV15` reader - REG_AUTOCMD_DEV_15"]
pub type Regautocmddev15R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV15` writer - REG_AUTOCMD_DEV_15"]
pub type Regautocmddev15W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_8"]
    #[inline(always)]
    pub fn regautocmddev8(&self) -> Regautocmddev8R {
        Regautocmddev8R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_9"]
    #[inline(always)]
    pub fn regautocmddev9(&self) -> Regautocmddev9R {
        Regautocmddev9R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_10"]
    #[inline(always)]
    pub fn regautocmddev10(&self) -> Regautocmddev10R {
        Regautocmddev10R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_11"]
    #[inline(always)]
    pub fn regautocmddev11(&self) -> Regautocmddev11R {
        Regautocmddev11R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_12"]
    #[inline(always)]
    pub fn regautocmddev12(&self) -> Regautocmddev12R {
        Regautocmddev12R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_13"]
    #[inline(always)]
    pub fn regautocmddev13(&self) -> Regautocmddev13R {
        Regautocmddev13R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_14"]
    #[inline(always)]
    pub fn regautocmddev14(&self) -> Regautocmddev14R {
        Regautocmddev14R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_15"]
    #[inline(always)]
    pub fn regautocmddev15(&self) -> Regautocmddev15R {
        Regautocmddev15R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_8"]
    #[inline(always)]
    pub fn regautocmddev8(&mut self) -> Regautocmddev8W<I3ccontrol044Spec> {
        Regautocmddev8W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_9"]
    #[inline(always)]
    pub fn regautocmddev9(&mut self) -> Regautocmddev9W<I3ccontrol044Spec> {
        Regautocmddev9W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_10"]
    #[inline(always)]
    pub fn regautocmddev10(&mut self) -> Regautocmddev10W<I3ccontrol044Spec> {
        Regautocmddev10W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_11"]
    #[inline(always)]
    pub fn regautocmddev11(&mut self) -> Regautocmddev11W<I3ccontrol044Spec> {
        Regautocmddev11W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_12"]
    #[inline(always)]
    pub fn regautocmddev12(&mut self) -> Regautocmddev12W<I3ccontrol044Spec> {
        Regautocmddev12W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_13"]
    #[inline(always)]
    pub fn regautocmddev13(&mut self) -> Regautocmddev13W<I3ccontrol044Spec> {
        Regautocmddev13W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_14"]
    #[inline(always)]
    pub fn regautocmddev14(&mut self) -> Regautocmddev14W<I3ccontrol044Spec> {
        Regautocmddev14W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_15"]
    #[inline(always)]
    pub fn regautocmddev15(&mut self) -> Regautocmddev15W<I3ccontrol044Spec> {
        Regautocmddev15W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_044\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol044::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol044::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol044Spec;
impl crate::RegisterSpec for I3ccontrol044Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol044::R`](R) reader structure"]
impl crate::Readable for I3ccontrol044Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol044::W`](W) writer structure"]
impl crate::Writable for I3ccontrol044Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL044 to value 0"]
impl crate::Resettable for I3ccontrol044Spec {}
