#[doc = "Register `I3CCONTROL020` reader"]
pub type R = crate::R<I3ccontrol020Spec>;
#[doc = "Register `I3CCONTROL020` writer"]
pub type W = crate::W<I3ccontrol020Spec>;
#[doc = "Field `REGAUTOCMDMODE0` reader - REG_AUTOCMD_MODE_0"]
pub type Regautocmdmode0R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE0` writer - REG_AUTOCMD_MODE_0"]
pub type Regautocmdmode0W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE0` reader - REG_AUTOCMD_VALUE_0"]
pub type Regautocmdvalue0R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE0` writer - REG_AUTOCMD_VALUE_0"]
pub type Regautocmdvalue0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK0` reader - REG_AUTOCMD_MASK_0"]
pub type Regautocmdmask0R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK0` writer - REG_AUTOCMD_MASK_0"]
pub type Regautocmdmask0W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_0"]
    #[inline(always)]
    pub fn regautocmdmode0(&self) -> Regautocmdmode0R {
        Regautocmdmode0R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_0"]
    #[inline(always)]
    pub fn regautocmdvalue0(&self) -> Regautocmdvalue0R {
        Regautocmdvalue0R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_0"]
    #[inline(always)]
    pub fn regautocmdmask0(&self) -> Regautocmdmask0R {
        Regautocmdmask0R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_0"]
    #[inline(always)]
    pub fn regautocmdmode0(&mut self) -> Regautocmdmode0W<I3ccontrol020Spec> {
        Regautocmdmode0W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_0"]
    #[inline(always)]
    pub fn regautocmdvalue0(&mut self) -> Regautocmdvalue0W<I3ccontrol020Spec> {
        Regautocmdvalue0W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_0"]
    #[inline(always)]
    pub fn regautocmdmask0(&mut self) -> Regautocmdmask0W<I3ccontrol020Spec> {
        Regautocmdmask0W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_0\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol020::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol020::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol020Spec;
impl crate::RegisterSpec for I3ccontrol020Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol020::R`](R) reader structure"]
impl crate::Readable for I3ccontrol020Spec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol020::W`](W) writer structure"]
impl crate::Writable for I3ccontrol020Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL020 to value 0xff00"]
impl crate::Resettable for I3ccontrol020Spec {
    const RESET_VALUE: u32 = 0xff00;
}
