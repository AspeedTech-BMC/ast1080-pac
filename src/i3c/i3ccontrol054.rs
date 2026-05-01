#[doc = "Register `I3CCONTROL054` reader"]
pub type R = crate::R<I3ccontrol054Spec>;
#[doc = "Register `I3CCONTROL054` writer"]
pub type W = crate::W<I3ccontrol054Spec>;
#[doc = "Field `REGAUTOCMDDEV40` reader - REG_AUTOCMD_DEV_40"]
pub type Regautocmddev40R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV40` writer - REG_AUTOCMD_DEV_40"]
pub type Regautocmddev40W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV41` reader - REG_AUTOCMD_DEV_41"]
pub type Regautocmddev41R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV41` writer - REG_AUTOCMD_DEV_41"]
pub type Regautocmddev41W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV42` reader - REG_AUTOCMD_DEV_42"]
pub type Regautocmddev42R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV42` writer - REG_AUTOCMD_DEV_42"]
pub type Regautocmddev42W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV43` reader - REG_AUTOCMD_DEV_43"]
pub type Regautocmddev43R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV43` writer - REG_AUTOCMD_DEV_43"]
pub type Regautocmddev43W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV44` reader - REG_AUTOCMD_DEV_44"]
pub type Regautocmddev44R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV44` writer - REG_AUTOCMD_DEV_44"]
pub type Regautocmddev44W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV45` reader - REG_AUTOCMD_DEV_45"]
pub type Regautocmddev45R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV45` writer - REG_AUTOCMD_DEV_45"]
pub type Regautocmddev45W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV46` reader - REG_AUTOCMD_DEV_46"]
pub type Regautocmddev46R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV46` writer - REG_AUTOCMD_DEV_46"]
pub type Regautocmddev46W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV47` reader - REG_AUTOCMD_DEV_47"]
pub type Regautocmddev47R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV47` writer - REG_AUTOCMD_DEV_47"]
pub type Regautocmddev47W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_40"]
    #[inline(always)]
    pub fn regautocmddev40(&self) -> Regautocmddev40R {
        Regautocmddev40R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_41"]
    #[inline(always)]
    pub fn regautocmddev41(&self) -> Regautocmddev41R {
        Regautocmddev41R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_42"]
    #[inline(always)]
    pub fn regautocmddev42(&self) -> Regautocmddev42R {
        Regautocmddev42R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_43"]
    #[inline(always)]
    pub fn regautocmddev43(&self) -> Regautocmddev43R {
        Regautocmddev43R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_44"]
    #[inline(always)]
    pub fn regautocmddev44(&self) -> Regautocmddev44R {
        Regautocmddev44R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_45"]
    #[inline(always)]
    pub fn regautocmddev45(&self) -> Regautocmddev45R {
        Regautocmddev45R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_46"]
    #[inline(always)]
    pub fn regautocmddev46(&self) -> Regautocmddev46R {
        Regautocmddev46R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_47"]
    #[inline(always)]
    pub fn regautocmddev47(&self) -> Regautocmddev47R {
        Regautocmddev47R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_40"]
    #[inline(always)]
    pub fn regautocmddev40(&mut self) -> Regautocmddev40W<I3ccontrol054Spec> {
        Regautocmddev40W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_41"]
    #[inline(always)]
    pub fn regautocmddev41(&mut self) -> Regautocmddev41W<I3ccontrol054Spec> {
        Regautocmddev41W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_42"]
    #[inline(always)]
    pub fn regautocmddev42(&mut self) -> Regautocmddev42W<I3ccontrol054Spec> {
        Regautocmddev42W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_43"]
    #[inline(always)]
    pub fn regautocmddev43(&mut self) -> Regautocmddev43W<I3ccontrol054Spec> {
        Regautocmddev43W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_44"]
    #[inline(always)]
    pub fn regautocmddev44(&mut self) -> Regautocmddev44W<I3ccontrol054Spec> {
        Regautocmddev44W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_45"]
    #[inline(always)]
    pub fn regautocmddev45(&mut self) -> Regautocmddev45W<I3ccontrol054Spec> {
        Regautocmddev45W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_46"]
    #[inline(always)]
    pub fn regautocmddev46(&mut self) -> Regautocmddev46W<I3ccontrol054Spec> {
        Regautocmddev46W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_47"]
    #[inline(always)]
    pub fn regautocmddev47(&mut self) -> Regautocmddev47W<I3ccontrol054Spec> {
        Regautocmddev47W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_054\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol054::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol054::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol054Spec;
impl crate::RegisterSpec for I3ccontrol054Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol054::R`](R) reader structure"]
impl crate::Readable for I3ccontrol054Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol054::W`](W) writer structure"]
impl crate::Writable for I3ccontrol054Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL054 to value 0"]
impl crate::Resettable for I3ccontrol054Spec {}
