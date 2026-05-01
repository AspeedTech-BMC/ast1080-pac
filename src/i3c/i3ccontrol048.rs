#[doc = "Register `I3CCONTROL048` reader"]
pub type R = crate::R<I3ccontrol048Spec>;
#[doc = "Register `I3CCONTROL048` writer"]
pub type W = crate::W<I3ccontrol048Spec>;
#[doc = "Field `REGAUTOCMDDEV16` reader - REG_AUTOCMD_DEV_16"]
pub type Regautocmddev16R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV16` writer - REG_AUTOCMD_DEV_16"]
pub type Regautocmddev16W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV17` reader - REG_AUTOCMD_DEV_17"]
pub type Regautocmddev17R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV17` writer - REG_AUTOCMD_DEV_17"]
pub type Regautocmddev17W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV18` reader - REG_AUTOCMD_DEV_18"]
pub type Regautocmddev18R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV18` writer - REG_AUTOCMD_DEV_18"]
pub type Regautocmddev18W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV19` reader - REG_AUTOCMD_DEV_19"]
pub type Regautocmddev19R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV19` writer - REG_AUTOCMD_DEV_19"]
pub type Regautocmddev19W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV20` reader - REG_AUTOCMD_DEV_20"]
pub type Regautocmddev20R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV20` writer - REG_AUTOCMD_DEV_20"]
pub type Regautocmddev20W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV21` reader - REG_AUTOCMD_DEV_21"]
pub type Regautocmddev21R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV21` writer - REG_AUTOCMD_DEV_21"]
pub type Regautocmddev21W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV22` reader - REG_AUTOCMD_DEV_22"]
pub type Regautocmddev22R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV22` writer - REG_AUTOCMD_DEV_22"]
pub type Regautocmddev22W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV23` reader - REG_AUTOCMD_DEV_23"]
pub type Regautocmddev23R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV23` writer - REG_AUTOCMD_DEV_23"]
pub type Regautocmddev23W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_16"]
    #[inline(always)]
    pub fn regautocmddev16(&self) -> Regautocmddev16R {
        Regautocmddev16R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_17"]
    #[inline(always)]
    pub fn regautocmddev17(&self) -> Regautocmddev17R {
        Regautocmddev17R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_18"]
    #[inline(always)]
    pub fn regautocmddev18(&self) -> Regautocmddev18R {
        Regautocmddev18R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_19"]
    #[inline(always)]
    pub fn regautocmddev19(&self) -> Regautocmddev19R {
        Regautocmddev19R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_20"]
    #[inline(always)]
    pub fn regautocmddev20(&self) -> Regautocmddev20R {
        Regautocmddev20R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_21"]
    #[inline(always)]
    pub fn regautocmddev21(&self) -> Regautocmddev21R {
        Regautocmddev21R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_22"]
    #[inline(always)]
    pub fn regautocmddev22(&self) -> Regautocmddev22R {
        Regautocmddev22R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_23"]
    #[inline(always)]
    pub fn regautocmddev23(&self) -> Regautocmddev23R {
        Regautocmddev23R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_16"]
    #[inline(always)]
    pub fn regautocmddev16(&mut self) -> Regautocmddev16W<I3ccontrol048Spec> {
        Regautocmddev16W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_17"]
    #[inline(always)]
    pub fn regautocmddev17(&mut self) -> Regautocmddev17W<I3ccontrol048Spec> {
        Regautocmddev17W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_18"]
    #[inline(always)]
    pub fn regautocmddev18(&mut self) -> Regautocmddev18W<I3ccontrol048Spec> {
        Regautocmddev18W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_19"]
    #[inline(always)]
    pub fn regautocmddev19(&mut self) -> Regautocmddev19W<I3ccontrol048Spec> {
        Regautocmddev19W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_20"]
    #[inline(always)]
    pub fn regautocmddev20(&mut self) -> Regautocmddev20W<I3ccontrol048Spec> {
        Regautocmddev20W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_21"]
    #[inline(always)]
    pub fn regautocmddev21(&mut self) -> Regautocmddev21W<I3ccontrol048Spec> {
        Regautocmddev21W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_22"]
    #[inline(always)]
    pub fn regautocmddev22(&mut self) -> Regautocmddev22W<I3ccontrol048Spec> {
        Regautocmddev22W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_23"]
    #[inline(always)]
    pub fn regautocmddev23(&mut self) -> Regautocmddev23W<I3ccontrol048Spec> {
        Regautocmddev23W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_048\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol048::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol048::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol048Spec;
impl crate::RegisterSpec for I3ccontrol048Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol048::R`](R) reader structure"]
impl crate::Readable for I3ccontrol048Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol048::W`](W) writer structure"]
impl crate::Writable for I3ccontrol048Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL048 to value 0"]
impl crate::Resettable for I3ccontrol048Spec {}
