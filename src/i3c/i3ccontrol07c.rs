#[doc = "Register `I3CCONTROL07C` reader"]
pub type R = crate::R<I3ccontrol07cSpec>;
#[doc = "Register `I3CCONTROL07C` writer"]
pub type W = crate::W<I3ccontrol07cSpec>;
#[doc = "Field `REGAUTOCMDDEV120` reader - REG_AUTOCMD_DEV_120"]
pub type Regautocmddev120R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV120` writer - REG_AUTOCMD_DEV_120"]
pub type Regautocmddev120W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV121` reader - REG_AUTOCMD_DEV_121"]
pub type Regautocmddev121R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV121` writer - REG_AUTOCMD_DEV_121"]
pub type Regautocmddev121W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV122` reader - REG_AUTOCMD_DEV_122"]
pub type Regautocmddev122R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV122` writer - REG_AUTOCMD_DEV_122"]
pub type Regautocmddev122W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV123` reader - REG_AUTOCMD_DEV_123"]
pub type Regautocmddev123R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV123` writer - REG_AUTOCMD_DEV_123"]
pub type Regautocmddev123W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV124` reader - REG_AUTOCMD_DEV_124"]
pub type Regautocmddev124R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV124` writer - REG_AUTOCMD_DEV_124"]
pub type Regautocmddev124W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV125` reader - REG_AUTOCMD_DEV_125"]
pub type Regautocmddev125R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV125` writer - REG_AUTOCMD_DEV_125"]
pub type Regautocmddev125W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV126` reader - REG_AUTOCMD_DEV_126"]
pub type Regautocmddev126R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV126` writer - REG_AUTOCMD_DEV_126"]
pub type Regautocmddev126W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV127` reader - REG_AUTOCMD_DEV_127"]
pub type Regautocmddev127R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV127` writer - REG_AUTOCMD_DEV_127"]
pub type Regautocmddev127W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_120"]
    #[inline(always)]
    pub fn regautocmddev120(&self) -> Regautocmddev120R {
        Regautocmddev120R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_121"]
    #[inline(always)]
    pub fn regautocmddev121(&self) -> Regautocmddev121R {
        Regautocmddev121R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_122"]
    #[inline(always)]
    pub fn regautocmddev122(&self) -> Regautocmddev122R {
        Regautocmddev122R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_123"]
    #[inline(always)]
    pub fn regautocmddev123(&self) -> Regautocmddev123R {
        Regautocmddev123R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_124"]
    #[inline(always)]
    pub fn regautocmddev124(&self) -> Regautocmddev124R {
        Regautocmddev124R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_125"]
    #[inline(always)]
    pub fn regautocmddev125(&self) -> Regautocmddev125R {
        Regautocmddev125R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_126"]
    #[inline(always)]
    pub fn regautocmddev126(&self) -> Regautocmddev126R {
        Regautocmddev126R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_127"]
    #[inline(always)]
    pub fn regautocmddev127(&self) -> Regautocmddev127R {
        Regautocmddev127R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_120"]
    #[inline(always)]
    pub fn regautocmddev120(&mut self) -> Regautocmddev120W<I3ccontrol07cSpec> {
        Regautocmddev120W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_121"]
    #[inline(always)]
    pub fn regautocmddev121(&mut self) -> Regautocmddev121W<I3ccontrol07cSpec> {
        Regautocmddev121W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_122"]
    #[inline(always)]
    pub fn regautocmddev122(&mut self) -> Regautocmddev122W<I3ccontrol07cSpec> {
        Regautocmddev122W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_123"]
    #[inline(always)]
    pub fn regautocmddev123(&mut self) -> Regautocmddev123W<I3ccontrol07cSpec> {
        Regautocmddev123W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_124"]
    #[inline(always)]
    pub fn regautocmddev124(&mut self) -> Regautocmddev124W<I3ccontrol07cSpec> {
        Regautocmddev124W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_125"]
    #[inline(always)]
    pub fn regautocmddev125(&mut self) -> Regautocmddev125W<I3ccontrol07cSpec> {
        Regautocmddev125W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_126"]
    #[inline(always)]
    pub fn regautocmddev126(&mut self) -> Regautocmddev126W<I3ccontrol07cSpec> {
        Regautocmddev126W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_127"]
    #[inline(always)]
    pub fn regautocmddev127(&mut self) -> Regautocmddev127W<I3ccontrol07cSpec> {
        Regautocmddev127W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_07C\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol07c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol07c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol07cSpec;
impl crate::RegisterSpec for I3ccontrol07cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol07c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol07cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol07c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol07cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL07C to value 0"]
impl crate::Resettable for I3ccontrol07cSpec {}
