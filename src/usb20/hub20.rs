#[doc = "Register `HUB20` reader"]
pub type R = crate::R<Hub20Spec>;
#[doc = "Register `HUB20` writer"]
pub type W = crate::W<Hub20Spec>;
#[doc = "Field `EnblRootHUBCtrlSwRst` reader - Enable Root HUB Controller software Reset"]
pub type EnblRootHubctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblRootHUBCtrlSwRst` writer - Enable Root HUB Controller software Reset"]
pub type EnblRootHubctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev1CtrlSwRst` reader - Enable Device #1 Controller software Reset"]
pub type EnblDev1ctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDev1CtrlSwRst` writer - Enable Device #1 Controller software Reset"]
pub type EnblDev1ctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev2CtrlSwRst` reader - Enable Device #2 Controller software Reset"]
pub type EnblDev2ctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDev2CtrlSwRst` writer - Enable Device #2 Controller software Reset"]
pub type EnblDev2ctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev3CtrlSwRst` reader - Enable Device #3 Controller software Reset"]
pub type EnblDev3ctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDev3CtrlSwRst` writer - Enable Device #3 Controller software Reset"]
pub type EnblDev3ctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev4CtrlSwRst` reader - Enable Device #4 Controller software Reset"]
pub type EnblDev4ctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDev4CtrlSwRst` writer - Enable Device #4 Controller software Reset"]
pub type EnblDev4ctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev5CtrlSwRst` reader - Enable Device #5 Controller software Reset"]
pub type EnblDev5ctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDev5CtrlSwRst` writer - Enable Device #5 Controller software Reset"]
pub type EnblDev5ctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev6CtrlSwRst` reader - Enable Device #6 Controller software Reset"]
pub type EnblDev6ctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDev6CtrlSwRst` writer - Enable Device #6 Controller software Reset"]
pub type EnblDev6ctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDev7CtrlSwRst` reader - Enable Device #7 Controller software Reset"]
pub type EnblDev7ctrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDev7CtrlSwRst` writer - Enable Device #7 Controller software Reset"]
pub type EnblDev7ctrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblDMACtrlSwRst` reader - Enable DMA Controller software Reset"]
pub type EnblDmactrlSwRstR = crate::BitReader;
#[doc = "Field `EnblDMACtrlSwRst` writer - Enable DMA Controller software Reset"]
pub type EnblDmactrlSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EnblProgrammableEndpointPoolSwRst` reader - Enable Programmable Endpoint Pool software Reset"]
pub type EnblProgrammableEndpointPoolSwRstR = crate::BitReader;
#[doc = "Field `EnblProgrammableEndpointPoolSwRst` writer - Enable Programmable Endpoint Pool software Reset"]
pub type EnblProgrammableEndpointPoolSwRstW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `Reserved0` reader - Reserved (0)"]
pub type Reserved0R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bit 0 - Enable Root HUB Controller software Reset"]
    #[inline(always)]
    pub fn enbl_root_hubctrl_sw_rst(&self) -> EnblRootHubctrlSwRstR {
        EnblRootHubctrlSwRstR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable Device #1 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev1ctrl_sw_rst(&self) -> EnblDev1ctrlSwRstR {
        EnblDev1ctrlSwRstR::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable Device #2 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev2ctrl_sw_rst(&self) -> EnblDev2ctrlSwRstR {
        EnblDev2ctrlSwRstR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable Device #3 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev3ctrl_sw_rst(&self) -> EnblDev3ctrlSwRstR {
        EnblDev3ctrlSwRstR::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable Device #4 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev4ctrl_sw_rst(&self) -> EnblDev4ctrlSwRstR {
        EnblDev4ctrlSwRstR::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable Device #5 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev5ctrl_sw_rst(&self) -> EnblDev5ctrlSwRstR {
        EnblDev5ctrlSwRstR::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable Device #6 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev6ctrl_sw_rst(&self) -> EnblDev6ctrlSwRstR {
        EnblDev6ctrlSwRstR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable Device #7 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev7ctrl_sw_rst(&self) -> EnblDev7ctrlSwRstR {
        EnblDev7ctrlSwRstR::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable DMA Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dmactrl_sw_rst(&self) -> EnblDmactrlSwRstR {
        EnblDmactrlSwRstR::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable Programmable Endpoint Pool software Reset"]
    #[inline(always)]
    pub fn enbl_programmable_endpoint_pool_sw_rst(&self) -> EnblProgrammableEndpointPoolSwRstR {
        EnblProgrammableEndpointPoolSwRstR::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bits 10:31 - Reserved (0)"]
    #[inline(always)]
    pub fn reserved0(&self) -> Reserved0R {
        Reserved0R::new((self.bits >> 10) & 0x003f_ffff)
    }
}
impl W {
    #[doc = "Bit 0 - Enable Root HUB Controller software Reset"]
    #[inline(always)]
    pub fn enbl_root_hubctrl_sw_rst(&mut self) -> EnblRootHubctrlSwRstW<Hub20Spec> {
        EnblRootHubctrlSwRstW::new(self, 0)
    }
    #[doc = "Bit 1 - Enable Device #1 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev1ctrl_sw_rst(&mut self) -> EnblDev1ctrlSwRstW<Hub20Spec> {
        EnblDev1ctrlSwRstW::new(self, 1)
    }
    #[doc = "Bit 2 - Enable Device #2 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev2ctrl_sw_rst(&mut self) -> EnblDev2ctrlSwRstW<Hub20Spec> {
        EnblDev2ctrlSwRstW::new(self, 2)
    }
    #[doc = "Bit 3 - Enable Device #3 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev3ctrl_sw_rst(&mut self) -> EnblDev3ctrlSwRstW<Hub20Spec> {
        EnblDev3ctrlSwRstW::new(self, 3)
    }
    #[doc = "Bit 4 - Enable Device #4 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev4ctrl_sw_rst(&mut self) -> EnblDev4ctrlSwRstW<Hub20Spec> {
        EnblDev4ctrlSwRstW::new(self, 4)
    }
    #[doc = "Bit 5 - Enable Device #5 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev5ctrl_sw_rst(&mut self) -> EnblDev5ctrlSwRstW<Hub20Spec> {
        EnblDev5ctrlSwRstW::new(self, 5)
    }
    #[doc = "Bit 6 - Enable Device #6 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev6ctrl_sw_rst(&mut self) -> EnblDev6ctrlSwRstW<Hub20Spec> {
        EnblDev6ctrlSwRstW::new(self, 6)
    }
    #[doc = "Bit 7 - Enable Device #7 Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dev7ctrl_sw_rst(&mut self) -> EnblDev7ctrlSwRstW<Hub20Spec> {
        EnblDev7ctrlSwRstW::new(self, 7)
    }
    #[doc = "Bit 8 - Enable DMA Controller software Reset"]
    #[inline(always)]
    pub fn enbl_dmactrl_sw_rst(&mut self) -> EnblDmactrlSwRstW<Hub20Spec> {
        EnblDmactrlSwRstW::new(self, 8)
    }
    #[doc = "Bit 9 - Enable Programmable Endpoint Pool software Reset"]
    #[inline(always)]
    pub fn enbl_programmable_endpoint_pool_sw_rst(
        &mut self,
    ) -> EnblProgrammableEndpointPoolSwRstW<Hub20Spec> {
        EnblProgrammableEndpointPoolSwRstW::new(self, 9)
    }
}
#[doc = "Device Controller Soft Reset Enable Register\n\nYou can [`read`](crate::Reg::read) this register and get [`hub20::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hub20::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct Hub20Spec;
impl crate::RegisterSpec for Hub20Spec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hub20::R`](R) reader structure"]
impl crate::Readable for Hub20Spec {}
#[doc = "`write(|w| ..)` method takes [`hub20::W`](W) writer structure"]
impl crate::Writable for Hub20Spec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUB20 to value 0x03ff"]
impl crate::Resettable for Hub20Spec {
    const RESET_VALUE: u32 = 0x03ff;
}
