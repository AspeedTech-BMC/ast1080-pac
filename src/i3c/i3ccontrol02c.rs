#[doc = "Register `I3CCONTROL02C` reader"]
pub type R = crate::R<I3ccontrol02cSpec>;
#[doc = "Register `I3CCONTROL02C` writer"]
pub type W = crate::W<I3ccontrol02cSpec>;
#[doc = "Field `REGAUTOCMDMODE3` reader - REG_AUTOCMD_MODE_3"]
pub type Regautocmdmode3R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMODE3` writer - REG_AUTOCMD_MODE_3"]
pub type Regautocmdmode3W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `Reserved1` reader - reserved"]
pub type Reserved1R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE3` reader - REG_AUTOCMD_VALUE_3"]
pub type Regautocmdvalue3R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDVALUE3` writer - REG_AUTOCMD_VALUE_3"]
pub type Regautocmdvalue3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `REGAUTOCMDMASK3` reader - REG_AUTOCMD_MASK_3"]
pub type Regautocmdmask3R = crate::FieldReader;
#[doc = "Field `REGAUTOCMDMASK3` writer - REG_AUTOCMD_MASK_3"]
pub type Regautocmdmask3W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_3"]
    #[inline(always)]
    pub fn regautocmdmode3(&self) -> Regautocmdmode3R {
        Regautocmdmode3R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:7 - reserved"]
    #[inline(always)]
    pub fn reserved1(&self) -> Reserved1R {
        Reserved1R::new(((self.bits >> 3) & 0x1f) as u8)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_3"]
    #[inline(always)]
    pub fn regautocmdvalue3(&self) -> Regautocmdvalue3R {
        Regautocmdvalue3R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_3"]
    #[inline(always)]
    pub fn regautocmdmask3(&self) -> Regautocmdmask3R {
        Regautocmdmask3R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:2 - REG_AUTOCMD_MODE_3"]
    #[inline(always)]
    pub fn regautocmdmode3(&mut self) -> Regautocmdmode3W<I3ccontrol02cSpec> {
        Regautocmdmode3W::new(self, 0)
    }
    #[doc = "Bits 8:15 - REG_AUTOCMD_VALUE_3"]
    #[inline(always)]
    pub fn regautocmdvalue3(&mut self) -> Regautocmdvalue3W<I3ccontrol02cSpec> {
        Regautocmdvalue3W::new(self, 8)
    }
    #[doc = "Bits 16:23 - REG_AUTOCMD_MASK_3"]
    #[inline(always)]
    pub fn regautocmdmask3(&mut self) -> Regautocmdmask3W<I3ccontrol02cSpec> {
        Regautocmdmask3W::new(self, 16)
    }
}
#[doc = "I3C\\_AUTOCMD\\_3\n\nYou can [`read`](crate::Reg::read) this register and get [`i3ccontrol02c::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`i3ccontrol02c::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct I3ccontrol02cSpec;
impl crate::RegisterSpec for I3ccontrol02cSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`i3ccontrol02c::R`](R) reader structure"]
impl crate::Readable for I3ccontrol02cSpec {}
#[doc = "`write(|w| ..)` method takes [`i3ccontrol02c::W`](W) writer structure"]
impl crate::Writable for I3ccontrol02cSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets I3CCONTROL02C to value 0xff00"]
impl crate::Resettable for I3ccontrol02cSpec {
    const RESET_VALUE: u32 = 0xff00;
}
