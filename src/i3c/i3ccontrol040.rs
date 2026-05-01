#[doc = "Register `I3CCONTROL040` reader"]
pub type R = crate::R<I3ccontrol040Spec>;
#[doc = "Register `I3CCONTROL040` writer"]
pub type W = crate::W<I3ccontrol040Spec>;
#[doc = "Field `REGAUTOCMDDEV0` reader - REG_AUTOCMD_DEV_0"]
pub type Regautocmddev0R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV0` writer - REG_AUTOCMD_DEV_0"]
pub type Regautocmddev0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved7` reader - reserved"]
pub type Reserved7R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV1` reader - REG_AUTOCMD_DEV_1"]
pub type Regautocmddev1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV1` writer - REG_AUTOCMD_DEV_1"]
pub type Regautocmddev1W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved6` reader - reserved"]
pub type Reserved6R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV2` reader - REG_AUTOCMD_DEV_2"]
pub type Regautocmddev2R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV2` writer - REG_AUTOCMD_DEV_2"]
pub type Regautocmddev2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved5` reader - reserved"]
pub type Reserved5R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV3` reader - REG_AUTOCMD_DEV_3"]
pub type Regautocmddev3R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV3` writer - REG_AUTOCMD_DEV_3"]
pub type Regautocmddev3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved4` reader - reserved"]
pub type Reserved4R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV4` reader - REG_AUTOCMD_DEV_4"]
pub type Regautocmddev4R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV4` writer - REG_AUTOCMD_DEV_4"]
pub type Regautocmddev4W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved3` reader - reserved"]
pub type Reserved3R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV5` reader - REG_AUTOCMD_DEV_5"]
pub type Regautocmddev5R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV5` writer - REG_AUTOCMD_DEV_5"]
pub type Regautocmddev5W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved2` reader - reserved"]
pub type Reserved2R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV6` reader - REG_AUTOCMD_DEV_6"]
pub type Regautocmddev6R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV6` writer - REG_AUTOCMD_DEV_6"]
pub type Regautocmddev6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::BitReader;
#[doc = "Field `REGAUTOCMDDEV7` reader - REG_AUTOCMD_DEV_7"]
pub type Regautocmddev7R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDDEV7` writer - REG_AUTOCMD_DEV_7"]
pub type Regautocmddev7W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_0"]
    #[inline(always)]
    pub fn regautocmddev0(&self) -> Regautocmddev0R {
        Regautocmddev0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - reserved"]
    #[inline(always)]
    pub fn reserved7(&self) -> Reserved7R {
        Reserved7R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_1"]
    #[inline(always)]
    pub fn regautocmddev1(&self) -> Regautocmddev1R {
        Regautocmddev1R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bit 7 - reserved"]
    #[inline(always)]
    pub fn reserved6(&self) -> Reserved6R {
        Reserved6R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_2"]
    #[inline(always)]
    pub fn regautocmddev2(&self) -> Regautocmddev2R {
        Regautocmddev2R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - reserved"]
    #[inline(always)]
    pub fn reserved5(&self) -> Reserved5R {
        Reserved5R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_3"]
    #[inline(always)]
    pub fn regautocmddev3(&self) -> Regautocmddev3R {
        Regautocmddev3R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bit 15 - reserved"]
    #[inline(always)]
    pub fn reserved4(&self) -> Reserved4R {
        Reserved4R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_4"]
    #[inline(always)]
    pub fn regautocmddev4(&self) -> Regautocmddev4R {
        Regautocmddev4R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - reserved"]
    #[inline(always)]
    pub fn reserved3(&self) -> Reserved3R {
        Reserved3R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_5"]
    #[inline(always)]
    pub fn regautocmddev5(&self) -> Regautocmddev5R {
        Regautocmddev5R::new(((self.bits >> 20) & 7) as u8)
    }
    #[doc = "Bit 23 - reserved"]
    #[inline(always)]
    pub fn reserved2(&self) -> Reserved2R {
        Reserved2R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_6"]
    #[inline(always)]
    pub fn regautocmddev6(&self) -> Regautocmddev6R {
        Regautocmddev6R::new(((self.bits >> 24) & 7) as u8)
    }
    #[doc = "Bit 27 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_7"]
    #[inline(always)]
    pub fn regautocmddev7(&self) -> Regautocmddev7R {
        Regautocmddev7R::new(((self.bits >> 28) & 7) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_DEV_0"]
    #[inline(always)]
    pub fn regautocmddev0(&mut self) -> Regautocmddev0W<I3ccontrol040Spec> {
        Regautocmddev0W::new(self, 0)
    }
    #[doc = "Bits 4:6 - REG_AUTOCMD_DEV_1"]
    #[inline(always)]
    pub fn regautocmddev1(&mut self) -> Regautocmddev1W<I3ccontrol040Spec> {
        Regautocmddev1W::new(self, 4)
    }
    #[doc = "Bits 8:10 - REG_AUTOCMD_DEV_2"]
    #[inline(always)]
    pub fn regautocmddev2(&mut self) -> Regautocmddev2W<I3ccontrol040Spec> {
        Regautocmddev2W::new(self, 8)
    }
    #[doc = "Bits 12:14 - REG_AUTOCMD_DEV_3"]
    #[inline(always)]
    pub fn regautocmddev3(&mut self) -> Regautocmddev3W<I3ccontrol040Spec> {
        Regautocmddev3W::new(self, 12)
    }
    #[doc = "Bits 16:18 - REG_AUTOCMD_DEV_4"]
    #[inline(always)]
    pub fn regautocmddev4(&mut self) -> Regautocmddev4W<I3ccontrol040Spec> {
        Regautocmddev4W::new(self, 16)
    }
    #[doc = "Bits 20:22 - REG_AUTOCMD_DEV_5"]
    #[inline(always)]
    pub fn regautocmddev5(&mut self) -> Regautocmddev5W<I3ccontrol040Spec> {
        Regautocmddev5W::new(self, 20)
    }
    #[doc = "Bits 24:26 - REG_AUTOCMD_DEV_6"]
    #[inline(always)]
    pub fn regautocmddev6(&mut self) -> Regautocmddev6W<I3ccontrol040Spec> {
        Regautocmddev6W::new(self, 24)
    }
    #[doc = "Bits 28:30 - REG_AUTOCMD_DEV_7"]
    #[inline(always)]
    pub fn regautocmddev7(&mut self) -> Regautocmddev7W<I3ccontrol040Spec> {
        Regautocmddev7W::new(self, 28)
    }
}
#[doc = "I3C\\_AUTOCMD\\_SEL\\_040\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol040::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol040::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol040Spec;
impl crate::RegisterSpec for I3ccontrol040Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol040::R`](R) reader structure"]
impl crate::Readable for I3ccontrol040Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol040::W`](W) writer structure"]
impl crate::Writable for I3ccontrol040Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL040 to value 0"]
impl crate::Resettable for I3ccontrol040Spec {}
