#[doc = "Register `I3CCONTROL074` reader"]
pub type R = crate::R<I3ccontrol074Spec>;
#[doc = "Register `I3CCONTROL074` writer"]
pub type W = crate::W<I3ccontrol074Spec>;
#[doc = "Field `REGAUTOCMDDEV104` reader - REG_AUTOCMD_DEV_104"]
pub type Regautocmddev104R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV104` writer - REG_AUTOCMD_DEV_104"]
pub type Regautocmddev104W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV105` reader - REG_AUTOCMD_DEV_105"]
pub type Regautocmddev105R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV105` writer - REG_AUTOCMD_DEV_105"]
pub type Regautocmddev105W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV106` reader - REG_AUTOCMD_DEV_106"]
pub type Regautocmddev106R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV106` writer - REG_AUTOCMD_DEV_106"]
pub type Regautocmddev106W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV107` reader - REG_AUTOCMD_DEV_107"]
pub type Regautocmddev107R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV107` writer - REG_AUTOCMD_DEV_107"]
pub type Regautocmddev107W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV108` reader - REG_AUTOCMD_DEV_108"]
pub type Regautocmddev108R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV108` writer - REG_AUTOCMD_DEV_108"]
pub type Regautocmddev108W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV109` reader - REG_AUTOCMD_DEV_109"]
pub type Regautocmddev109R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV109` writer - REG_AUTOCMD_DEV_109"]
pub type Regautocmddev109W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV110` reader - REG_AUTOCMD_DEV_110"]
pub type Regautocmddev110R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV110` writer - REG_AUTOCMD_DEV_110"]
pub type Regautocmddev110W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV111` reader - REG_AUTOCMD_DEV_111"]
pub type Regautocmddev111R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV111` writer - REG_AUTOCMD_DEV_111"]
pub type Regautocmddev111W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_104"]
    #[inline(always)]
    pub fn regautocmddev104(&self) -> Regautocmddev104R {
        Regautocmddev104R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_105"]
    #[inline(always)]
    pub fn regautocmddev105(&self) -> Regautocmddev105R {
        Regautocmddev105R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_106"]
    #[inline(always)]
    pub fn regautocmddev106(&self) -> Regautocmddev106R {
        Regautocmddev106R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_107"]
    #[inline(always)]
    pub fn regautocmddev107(&self) -> Regautocmddev107R {
        Regautocmddev107R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_108"]
    #[inline(always)]
    pub fn regautocmddev108(&self) -> Regautocmddev108R {
        Regautocmddev108R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_109"]
    #[inline(always)]
    pub fn regautocmddev109(&self) -> Regautocmddev109R {
        Regautocmddev109R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_110"]
    #[inline(always)]
    pub fn regautocmddev110(&self) -> Regautocmddev110R {
        Regautocmddev110R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_111"]
    #[inline(always)]
    pub fn regautocmddev111(&self) -> Regautocmddev111R {
        Regautocmddev111R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_104"]
    #[inline(always)]
    pub fn regautocmddev104(&mut self) -> Regautocmddev104W<I3ccontrol074Spec> {
        Regautocmddev104W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_105"]
    #[inline(always)]
    pub fn regautocmddev105(&mut self) -> Regautocmddev105W<I3ccontrol074Spec> {
        Regautocmddev105W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_106"]
    #[inline(always)]
    pub fn regautocmddev106(&mut self) -> Regautocmddev106W<I3ccontrol074Spec> {
        Regautocmddev106W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_107"]
    #[inline(always)]
    pub fn regautocmddev107(&mut self) -> Regautocmddev107W<I3ccontrol074Spec> {
        Regautocmddev107W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_108"]
    #[inline(always)]
    pub fn regautocmddev108(&mut self) -> Regautocmddev108W<I3ccontrol074Spec> {
        Regautocmddev108W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_109"]
    #[inline(always)]
    pub fn regautocmddev109(&mut self) -> Regautocmddev109W<I3ccontrol074Spec> {
        Regautocmddev109W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_110"]
    #[inline(always)]
    pub fn regautocmddev110(&mut self) -> Regautocmddev110W<I3ccontrol074Spec> {
        Regautocmddev110W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_111"]
    #[inline(always)]
    pub fn regautocmddev111(&mut self) -> Regautocmddev111W<I3ccontrol074Spec> {
        Regautocmddev111W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_074\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol074::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol074::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol074Spec;
impl crate::RegisterSpec for I3ccontrol074Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol074::R`](R) reader structure"]
impl crate::Readable for I3ccontrol074Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol074::W`](W) writer structure"]
impl crate::Writable for I3ccontrol074Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL074 to value 0"]
impl crate::Resettable for I3ccontrol074Spec {}
