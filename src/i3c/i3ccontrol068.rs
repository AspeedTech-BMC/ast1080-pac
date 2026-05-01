#[doc = "Register `I3CCONTROL068` reader"]
pub type R = crate::R<I3ccontrol068Spec>;
#[doc = "Register `I3CCONTROL068` writer"]
pub type W = crate::W<I3ccontrol068Spec>;
#[doc = "Field `REGAUTOCMDDEV80` reader - REG_AUTOCMD_DEV_80"]
pub type Regautocmddev80R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV80` writer - REG_AUTOCMD_DEV_80"]
pub type Regautocmddev80W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV81` reader - REG_AUTOCMD_DEV_81"]
pub type Regautocmddev81R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV81` writer - REG_AUTOCMD_DEV_81"]
pub type Regautocmddev81W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV82` reader - REG_AUTOCMD_DEV_82"]
pub type Regautocmddev82R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV82` writer - REG_AUTOCMD_DEV_82"]
pub type Regautocmddev82W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV83` reader - REG_AUTOCMD_DEV_83"]
pub type Regautocmddev83R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV83` writer - REG_AUTOCMD_DEV_83"]
pub type Regautocmddev83W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV84` reader - REG_AUTOCMD_DEV_84"]
pub type Regautocmddev84R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV84` writer - REG_AUTOCMD_DEV_84"]
pub type Regautocmddev84W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV85` reader - REG_AUTOCMD_DEV_85"]
pub type Regautocmddev85R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV85` writer - REG_AUTOCMD_DEV_85"]
pub type Regautocmddev85W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV86` reader - REG_AUTOCMD_DEV_86"]
pub type Regautocmddev86R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV86` writer - REG_AUTOCMD_DEV_86"]
pub type Regautocmddev86W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV87` reader - REG_AUTOCMD_DEV_87"]
pub type Regautocmddev87R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV87` writer - REG_AUTOCMD_DEV_87"]
pub type Regautocmddev87W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_80"]
    #[inline(always)]
    pub fn regautocmddev80(&self) -> Regautocmddev80R {
        Regautocmddev80R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_81"]
    #[inline(always)]
    pub fn regautocmddev81(&self) -> Regautocmddev81R {
        Regautocmddev81R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_82"]
    #[inline(always)]
    pub fn regautocmddev82(&self) -> Regautocmddev82R {
        Regautocmddev82R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_83"]
    #[inline(always)]
    pub fn regautocmddev83(&self) -> Regautocmddev83R {
        Regautocmddev83R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_84"]
    #[inline(always)]
    pub fn regautocmddev84(&self) -> Regautocmddev84R {
        Regautocmddev84R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_85"]
    #[inline(always)]
    pub fn regautocmddev85(&self) -> Regautocmddev85R {
        Regautocmddev85R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_86"]
    #[inline(always)]
    pub fn regautocmddev86(&self) -> Regautocmddev86R {
        Regautocmddev86R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_87"]
    #[inline(always)]
    pub fn regautocmddev87(&self) -> Regautocmddev87R {
        Regautocmddev87R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_80"]
    #[inline(always)]
    pub fn regautocmddev80(&mut self) -> Regautocmddev80W<I3ccontrol068Spec> {
        Regautocmddev80W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_81"]
    #[inline(always)]
    pub fn regautocmddev81(&mut self) -> Regautocmddev81W<I3ccontrol068Spec> {
        Regautocmddev81W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_82"]
    #[inline(always)]
    pub fn regautocmddev82(&mut self) -> Regautocmddev82W<I3ccontrol068Spec> {
        Regautocmddev82W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_83"]
    #[inline(always)]
    pub fn regautocmddev83(&mut self) -> Regautocmddev83W<I3ccontrol068Spec> {
        Regautocmddev83W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_84"]
    #[inline(always)]
    pub fn regautocmddev84(&mut self) -> Regautocmddev84W<I3ccontrol068Spec> {
        Regautocmddev84W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_85"]
    #[inline(always)]
    pub fn regautocmddev85(&mut self) -> Regautocmddev85W<I3ccontrol068Spec> {
        Regautocmddev85W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_86"]
    #[inline(always)]
    pub fn regautocmddev86(&mut self) -> Regautocmddev86W<I3ccontrol068Spec> {
        Regautocmddev86W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_87"]
    #[inline(always)]
    pub fn regautocmddev87(&mut self) -> Regautocmddev87W<I3ccontrol068Spec> {
        Regautocmddev87W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_068\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol068::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol068::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol068Spec;
impl crate::RegisterSpec for I3ccontrol068Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol068::R`](R) reader structure"]
impl crate::Readable for I3ccontrol068Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol068::W`](W) writer structure"]
impl crate::Writable for I3ccontrol068Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL068 to value 0"]
impl crate::Resettable for I3ccontrol068Spec {}
