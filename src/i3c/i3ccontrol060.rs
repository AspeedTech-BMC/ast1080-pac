#[doc = "Register `I3CCONTROL060` reader"]
pub type R = crate::R<I3ccontrol060Spec>;
#[doc = "Register `I3CCONTROL060` writer"]
pub type W = crate::W<I3ccontrol060Spec>;
#[doc = "Field `REGAUTOCMDDEV64` reader - REG_AUTOCMD_DEV_64"]
pub type Regautocmddev64R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV64` writer - REG_AUTOCMD_DEV_64"]
pub type Regautocmddev64W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV65` reader - REG_AUTOCMD_DEV_65"]
pub type Regautocmddev65R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV65` writer - REG_AUTOCMD_DEV_65"]
pub type Regautocmddev65W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV66` reader - REG_AUTOCMD_DEV_66"]
pub type Regautocmddev66R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV66` writer - REG_AUTOCMD_DEV_66"]
pub type Regautocmddev66W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV67` reader - REG_AUTOCMD_DEV_67"]
pub type Regautocmddev67R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV67` writer - REG_AUTOCMD_DEV_67"]
pub type Regautocmddev67W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV68` reader - REG_AUTOCMD_DEV_68"]
pub type Regautocmddev68R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV68` writer - REG_AUTOCMD_DEV_68"]
pub type Regautocmddev68W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV69` reader - REG_AUTOCMD_DEV_69"]
pub type Regautocmddev69R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV69` writer - REG_AUTOCMD_DEV_69"]
pub type Regautocmddev69W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV70` reader - REG_AUTOCMD_DEV_70"]
pub type Regautocmddev70R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV70` writer - REG_AUTOCMD_DEV_70"]
pub type Regautocmddev70W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV71` reader - REG_AUTOCMD_DEV_71"]
pub type Regautocmddev71R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV71` writer - REG_AUTOCMD_DEV_71"]
pub type Regautocmddev71W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_64"]
    #[inline(always)]
    pub fn regautocmddev64(&self) -> Regautocmddev64R {
        Regautocmddev64R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_65"]
    #[inline(always)]
    pub fn regautocmddev65(&self) -> Regautocmddev65R {
        Regautocmddev65R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_66"]
    #[inline(always)]
    pub fn regautocmddev66(&self) -> Regautocmddev66R {
        Regautocmddev66R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_67"]
    #[inline(always)]
    pub fn regautocmddev67(&self) -> Regautocmddev67R {
        Regautocmddev67R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_68"]
    #[inline(always)]
    pub fn regautocmddev68(&self) -> Regautocmddev68R {
        Regautocmddev68R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_69"]
    #[inline(always)]
    pub fn regautocmddev69(&self) -> Regautocmddev69R {
        Regautocmddev69R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_70"]
    #[inline(always)]
    pub fn regautocmddev70(&self) -> Regautocmddev70R {
        Regautocmddev70R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_71"]
    #[inline(always)]
    pub fn regautocmddev71(&self) -> Regautocmddev71R {
        Regautocmddev71R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_64"]
    #[inline(always)]
    pub fn regautocmddev64(&mut self) -> Regautocmddev64W<I3ccontrol060Spec> {
        Regautocmddev64W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_65"]
    #[inline(always)]
    pub fn regautocmddev65(&mut self) -> Regautocmddev65W<I3ccontrol060Spec> {
        Regautocmddev65W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_66"]
    #[inline(always)]
    pub fn regautocmddev66(&mut self) -> Regautocmddev66W<I3ccontrol060Spec> {
        Regautocmddev66W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_67"]
    #[inline(always)]
    pub fn regautocmddev67(&mut self) -> Regautocmddev67W<I3ccontrol060Spec> {
        Regautocmddev67W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_68"]
    #[inline(always)]
    pub fn regautocmddev68(&mut self) -> Regautocmddev68W<I3ccontrol060Spec> {
        Regautocmddev68W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_69"]
    #[inline(always)]
    pub fn regautocmddev69(&mut self) -> Regautocmddev69W<I3ccontrol060Spec> {
        Regautocmddev69W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_70"]
    #[inline(always)]
    pub fn regautocmddev70(&mut self) -> Regautocmddev70W<I3ccontrol060Spec> {
        Regautocmddev70W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_71"]
    #[inline(always)]
    pub fn regautocmddev71(&mut self) -> Regautocmddev71W<I3ccontrol060Spec> {
        Regautocmddev71W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_060\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol060::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol060::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol060Spec;
impl crate::RegisterSpec for I3ccontrol060Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol060::R`](R) reader structure"]
impl crate::Readable for I3ccontrol060Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol060::W`](W) writer structure"]
impl crate::Writable for I3ccontrol060Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL060 to value 0"]
impl crate::Resettable for I3ccontrol060Spec {}
