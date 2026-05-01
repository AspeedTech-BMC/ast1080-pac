#[doc = "Register `JTAG018` reader"]
pub type R = crate::R<Jtag018Spec>;
#[doc = "Register `JTAG018` writer"]
pub type W = crate::W<Jtag018Spec>;
#[doc = "Control of TRSTn.\n\nValue on reset: 1"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CtrlOfTrstn {
    #[doc = "1: TRSTn is high."]
    TrstnIsHigh = 1,
    #[doc = "0: TRSTn is low."]
    TrstnIsLow = 0,
}
impl From<CtrlOfTrstn> for bool {
    #[inline(always)]
    fn from(variant: CtrlOfTrstn) -> Self {
        variant as u8 != 0
    }
}
#[doc = "Field `CtrlOfTRSTn` reader - Control of TRSTn."]
pub type CtrlOfTrstnR = crate::BitReader<CtrlOfTrstn>;
impl CtrlOfTrstnR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CtrlOfTrstn {
        match self.bits {
            true => CtrlOfTrstn::TrstnIsHigh,
            false => CtrlOfTrstn::TrstnIsLow,
        }
    }
    #[doc = "TRSTn is high."]
    #[inline(always)]
    pub fn is_trstn_is_high(&self) -> bool {
        *self == CtrlOfTrstn::TrstnIsHigh
    }
    #[doc = "TRSTn is low."]
    #[inline(always)]
    pub fn is_trstn_is_low(&self) -> bool {
        *self == CtrlOfTrstn::TrstnIsLow
    }
}
#[doc = "Field `CtrlOfTRSTn` writer - Control of TRSTn."]
pub type CtrlOfTrstnW<'a, REG> = crate::BitWriter<'a, REG, CtrlOfTrstn>;
impl<'a, REG> CtrlOfTrstnW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
{
    #[doc = "TRSTn is high."]
    #[inline(always)]
    pub fn trstn_is_high(self) -> &'a mut crate::W<REG> {
        self.variant(CtrlOfTrstn::TrstnIsHigh)
    }
    #[doc = "TRSTn is low."]
    #[inline(always)]
    pub fn trstn_is_low(self) -> &'a mut crate::W<REG> {
        self.variant(CtrlOfTrstn::TrstnIsLow)
    }
}
impl R {
    #[doc = "Bit 31 - Control of TRSTn."]
    #[inline(always)]
    pub fn ctrl_of_trstn(&self) -> CtrlOfTrstnR {
        CtrlOfTrstnR::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - Control of TRSTn."]
    #[inline(always)]
    pub fn ctrl_of_trstn(&mut self) -> CtrlOfTrstnW<Jtag018Spec> {
        CtrlOfTrstnW::new(self, 31)
    }
}
#[doc = "Engine Control 1\n\nYou can [`read`](crate::Reg::read) this register and get [`jtag018::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`jtag018::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Jtag018Spec;
impl crate::RegisterSpec for Jtag018Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`jtag018::R`](R) reader structure"]
impl crate::Readable for Jtag018Spec {}
#[doc = "`write(|w| ..)` method takes [`jtag018::W`](W) writer structure"]
impl crate::Writable for Jtag018Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets JTAG018 to value 0x8000_0000"]
impl crate::Resettable for Jtag018Spec {
    const RESET_VALUE: u32 = 0x8000_0000;
}
