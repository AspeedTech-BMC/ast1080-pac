#[doc = "Register `I3CCONTROL06C` reader"]
pub type R = crate::R<I3ccontrol06cSpec>;
#[doc = "Register `I3CCONTROL06C` writer"]
pub type W = crate::W<I3ccontrol06cSpec>;
#[doc = "Field `REGAUTOCMDDEV88` reader - REG_AUTOCMD_DEV_88"]
pub type Regautocmddev88R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV88` writer - REG_AUTOCMD_DEV_88"]
pub type Regautocmddev88W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV89` reader - REG_AUTOCMD_DEV_89"]
pub type Regautocmddev89R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV89` writer - REG_AUTOCMD_DEV_89"]
pub type Regautocmddev89W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV90` reader - REG_AUTOCMD_DEV_90"]
pub type Regautocmddev90R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV90` writer - REG_AUTOCMD_DEV_90"]
pub type Regautocmddev90W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV91` reader - REG_AUTOCMD_DEV_91"]
pub type Regautocmddev91R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV91` writer - REG_AUTOCMD_DEV_91"]
pub type Regautocmddev91W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV92` reader - REG_AUTOCMD_DEV_92"]
pub type Regautocmddev92R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV92` writer - REG_AUTOCMD_DEV_92"]
pub type Regautocmddev92W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV93` reader - REG_AUTOCMD_DEV_93"]
pub type Regautocmddev93R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV93` writer - REG_AUTOCMD_DEV_93"]
pub type Regautocmddev93W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV94` reader - REG_AUTOCMD_DEV_94"]
pub type Regautocmddev94R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV94` writer - REG_AUTOCMD_DEV_94"]
pub type Regautocmddev94W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV95` reader - REG_AUTOCMD_DEV_95"]
pub type Regautocmddev95R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV95` writer - REG_AUTOCMD_DEV_95"]
pub type Regautocmddev95W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_88"]
    #[inline(always)]
    pub fn regautocmddev88(&self) -> Regautocmddev88R {
        Regautocmddev88R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_89"]
    #[inline(always)]
    pub fn regautocmddev89(&self) -> Regautocmddev89R {
        Regautocmddev89R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_90"]
    #[inline(always)]
    pub fn regautocmddev90(&self) -> Regautocmddev90R {
        Regautocmddev90R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_91"]
    #[inline(always)]
    pub fn regautocmddev91(&self) -> Regautocmddev91R {
        Regautocmddev91R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_92"]
    #[inline(always)]
    pub fn regautocmddev92(&self) -> Regautocmddev92R {
        Regautocmddev92R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_93"]
    #[inline(always)]
    pub fn regautocmddev93(&self) -> Regautocmddev93R {
        Regautocmddev93R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_94"]
    #[inline(always)]
    pub fn regautocmddev94(&self) -> Regautocmddev94R {
        Regautocmddev94R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_95"]
    #[inline(always)]
    pub fn regautocmddev95(&self) -> Regautocmddev95R {
        Regautocmddev95R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_88"]
    #[inline(always)]
    pub fn regautocmddev88(&mut self) -> Regautocmddev88W<I3ccontrol06cSpec> {
        Regautocmddev88W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_89"]
    #[inline(always)]
    pub fn regautocmddev89(&mut self) -> Regautocmddev89W<I3ccontrol06cSpec> {
        Regautocmddev89W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_90"]
    #[inline(always)]
    pub fn regautocmddev90(&mut self) -> Regautocmddev90W<I3ccontrol06cSpec> {
        Regautocmddev90W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_91"]
    #[inline(always)]
    pub fn regautocmddev91(&mut self) -> Regautocmddev91W<I3ccontrol06cSpec> {
        Regautocmddev91W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_92"]
    #[inline(always)]
    pub fn regautocmddev92(&mut self) -> Regautocmddev92W<I3ccontrol06cSpec> {
        Regautocmddev92W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_93"]
    #[inline(always)]
    pub fn regautocmddev93(&mut self) -> Regautocmddev93W<I3ccontrol06cSpec> {
        Regautocmddev93W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_94"]
    #[inline(always)]
    pub fn regautocmddev94(&mut self) -> Regautocmddev94W<I3ccontrol06cSpec> {
        Regautocmddev94W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_95"]
    #[inline(always)]
    pub fn regautocmddev95(&mut self) -> Regautocmddev95W<I3ccontrol06cSpec> {
        Regautocmddev95W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_06C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol06c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol06c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol06cSpec;
impl crate::RegisterSpec for I3ccontrol06cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol06c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol06cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol06c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol06cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL06C to value 0"]
impl crate::Resettable for I3ccontrol06cSpec {}
