#[doc = "Register `I3CCONTROL038` reader"]
pub type R = crate::R<I3ccontrol038Spec>;
#[doc = "Register `I3CCONTROL038` writer"]
pub type W = crate::W<I3ccontrol038Spec>;
#[doc = "Field `REGAUTOCMDMODE6` reader - REG_AUTOCMD_MODE_6"]
pub type Regautocmdmode6R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE6` writer - REG_AUTOCMD_MODE_6"]
pub type Regautocmdmode6W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE6` reader - REG_AUTOCMD_VALUE_6"]
pub type Regautocmdvalue6R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE6` writer - REG_AUTOCMD_VALUE_6"]
pub type Regautocmdvalue6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK6` reader - REG_AUTOCMD_MASK_6"]
pub type Regautocmdmask6R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK6` writer - REG_AUTOCMD_MASK_6"]
pub type Regautocmdmask6W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_6"]
    #[inline(always)]
    pub fn regautocmdmode6(&self) -> Regautocmdmode6R {
        Regautocmdmode6R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_6"]
    #[inline(always)]
    pub fn regautocmdvalue6(&self) -> Regautocmdvalue6R {
        Regautocmdvalue6R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_6"]
    #[inline(always)]
    pub fn regautocmdmask6(&self) -> Regautocmdmask6R {
        Regautocmdmask6R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_6"]
    #[inline(always)]
    pub fn regautocmdmode6(&mut self) -> Regautocmdmode6W<I3ccontrol038Spec> {
        Regautocmdmode6W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_6"]
    #[inline(always)]
    pub fn regautocmdvalue6(&mut self) -> Regautocmdvalue6W<I3ccontrol038Spec> {
        Regautocmdvalue6W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_6"]
    #[inline(always)]
    pub fn regautocmdmask6(&mut self) -> Regautocmdmask6W<I3ccontrol038Spec> {
        Regautocmdmask6W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_6\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol038::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol038::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol038Spec;
impl crate::RegisterSpec for I3ccontrol038Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol038::R`](R) reader structure"]
impl crate::Readable for I3ccontrol038Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol038::W`](W) writer structure"]
impl crate::Writable for I3ccontrol038Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL038 to value 0xff00"]
impl crate::Resettable for I3ccontrol038Spec {
    const RESET_VALUE: u32 = 0xff00;
}
