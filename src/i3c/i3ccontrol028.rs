#[doc = "Register `I3CCONTROL028` reader"]
pub type R = crate::R<I3ccontrol028Spec>;
#[doc = "Register `I3CCONTROL028` writer"]
pub type W = crate::W<I3ccontrol028Spec>;
#[doc = "Field `REGAUTOCMDMODE2` reader - REG_AUTOCMD_MODE_2"]
pub type Regautocmdmode2R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE2` writer - REG_AUTOCMD_MODE_2"]
pub type Regautocmdmode2W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE2` reader - REG_AUTOCMD_VALUE_2"]
pub type Regautocmdvalue2R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE2` writer - REG_AUTOCMD_VALUE_2"]
pub type Regautocmdvalue2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK2` reader - REG_AUTOCMD_MASK_2"]
pub type Regautocmdmask2R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK2` writer - REG_AUTOCMD_MASK_2"]
pub type Regautocmdmask2W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_2"]
    #[inline(always)]
    pub fn regautocmdmode2(&self) -> Regautocmdmode2R {
        Regautocmdmode2R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_2"]
    #[inline(always)]
    pub fn regautocmdvalue2(&self) -> Regautocmdvalue2R {
        Regautocmdvalue2R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_2"]
    #[inline(always)]
    pub fn regautocmdmask2(&self) -> Regautocmdmask2R {
        Regautocmdmask2R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_2"]
    #[inline(always)]
    pub fn regautocmdmode2(&mut self) -> Regautocmdmode2W<I3ccontrol028Spec> {
        Regautocmdmode2W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_2"]
    #[inline(always)]
    pub fn regautocmdvalue2(&mut self) -> Regautocmdvalue2W<I3ccontrol028Spec> {
        Regautocmdvalue2W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_2"]
    #[inline(always)]
    pub fn regautocmdmask2(&mut self) -> Regautocmdmask2W<I3ccontrol028Spec> {
        Regautocmdmask2W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_2\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol028::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol028::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol028Spec;
impl crate::RegisterSpec for I3ccontrol028Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol028::R`](R) reader structure"]
impl crate::Readable for I3ccontrol028Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol028::W`](W) writer structure"]
impl crate::Writable for I3ccontrol028Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL028 to value 0xff00"]
impl crate::Resettable for I3ccontrol028Spec {
    const RESET_VALUE: u32 = 0xff00;
}
