#[doc = "Register `I3CCONTROL064` reader"]
pub type R = crate::R<I3ccontrol064Spec>;
#[doc = "Register `I3CCONTROL064` writer"]
pub type W = crate::W<I3ccontrol064Spec>;
#[doc = "Field `REGAUTOCMDDEV72` reader - REG_AUTOCMD_DEV_72"]
pub type Regautocmddev72R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV72` writer - REG_AUTOCMD_DEV_72"]
pub type Regautocmddev72W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV73` reader - REG_AUTOCMD_DEV_73"]
pub type Regautocmddev73R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV73` writer - REG_AUTOCMD_DEV_73"]
pub type Regautocmddev73W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV74` reader - REG_AUTOCMD_DEV_74"]
pub type Regautocmddev74R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV74` writer - REG_AUTOCMD_DEV_74"]
pub type Regautocmddev74W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV75` reader - REG_AUTOCMD_DEV_75"]
pub type Regautocmddev75R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV75` writer - REG_AUTOCMD_DEV_75"]
pub type Regautocmddev75W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV76` reader - REG_AUTOCMD_DEV_76"]
pub type Regautocmddev76R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV76` writer - REG_AUTOCMD_DEV_76"]
pub type Regautocmddev76W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV77` reader - REG_AUTOCMD_DEV_77"]
pub type Regautocmddev77R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV77` writer - REG_AUTOCMD_DEV_77"]
pub type Regautocmddev77W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV78` reader - REG_AUTOCMD_DEV_78"]
pub type Regautocmddev78R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV78` writer - REG_AUTOCMD_DEV_78"]
pub type Regautocmddev78W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV79` reader - REG_AUTOCMD_DEV_79"]
pub type Regautocmddev79R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV79` writer - REG_AUTOCMD_DEV_79"]
pub type Regautocmddev79W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_72"]
    #[inline(always)]
    pub fn regautocmddev72(&self) -> Regautocmddev72R {
        Regautocmddev72R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_73"]
    #[inline(always)]
    pub fn regautocmddev73(&self) -> Regautocmddev73R {
        Regautocmddev73R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_74"]
    #[inline(always)]
    pub fn regautocmddev74(&self) -> Regautocmddev74R {
        Regautocmddev74R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_75"]
    #[inline(always)]
    pub fn regautocmddev75(&self) -> Regautocmddev75R {
        Regautocmddev75R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_76"]
    #[inline(always)]
    pub fn regautocmddev76(&self) -> Regautocmddev76R {
        Regautocmddev76R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_77"]
    #[inline(always)]
    pub fn regautocmddev77(&self) -> Regautocmddev77R {
        Regautocmddev77R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_78"]
    #[inline(always)]
    pub fn regautocmddev78(&self) -> Regautocmddev78R {
        Regautocmddev78R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_79"]
    #[inline(always)]
    pub fn regautocmddev79(&self) -> Regautocmddev79R {
        Regautocmddev79R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_72"]
    #[inline(always)]
    pub fn regautocmddev72(&mut self) -> Regautocmddev72W<I3ccontrol064Spec> {
        Regautocmddev72W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_73"]
    #[inline(always)]
    pub fn regautocmddev73(&mut self) -> Regautocmddev73W<I3ccontrol064Spec> {
        Regautocmddev73W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_74"]
    #[inline(always)]
    pub fn regautocmddev74(&mut self) -> Regautocmddev74W<I3ccontrol064Spec> {
        Regautocmddev74W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_75"]
    #[inline(always)]
    pub fn regautocmddev75(&mut self) -> Regautocmddev75W<I3ccontrol064Spec> {
        Regautocmddev75W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_76"]
    #[inline(always)]
    pub fn regautocmddev76(&mut self) -> Regautocmddev76W<I3ccontrol064Spec> {
        Regautocmddev76W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_77"]
    #[inline(always)]
    pub fn regautocmddev77(&mut self) -> Regautocmddev77W<I3ccontrol064Spec> {
        Regautocmddev77W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_78"]
    #[inline(always)]
    pub fn regautocmddev78(&mut self) -> Regautocmddev78W<I3ccontrol064Spec> {
        Regautocmddev78W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_79"]
    #[inline(always)]
    pub fn regautocmddev79(&mut self) -> Regautocmddev79W<I3ccontrol064Spec> {
        Regautocmddev79W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_064\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol064::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol064::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol064Spec;
impl crate::RegisterSpec for I3ccontrol064Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol064::R`](R) reader structure"]
impl crate::Readable for I3ccontrol064Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol064::W`](W) writer structure"]
impl crate::Writable for I3ccontrol064Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL064 to value 0"]
impl crate::Resettable for I3ccontrol064Spec {}
